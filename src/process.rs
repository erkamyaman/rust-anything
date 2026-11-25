//! Every outside program runs with stdin closed, in its own process group, under a time limit.

use std::ffi::OsStr;
use std::io::Read;
use std::path::Path;
use std::process::{Child, Command, ExitStatus, Stdio};
use std::time::{Duration, Instant};

use anyhow::{Context, Result, bail};

const KEEP: usize = 1 << 20;

pub struct Output {
    pub status: ExitStatus,
    pub stdout: Vec<u8>,
    pub stderr: Vec<u8>,
}

impl Output {
    pub fn success(&self) -> bool {
        self.status.success()
    }

    pub fn stdout_text(&self) -> String {
        String::from_utf8_lossy(&self.stdout).into_owned()
    }

    pub fn stderr_text(&self) -> String {
        String::from_utf8_lossy(&self.stderr).into_owned()
    }
}

/// `RUST_ANYTHING_COMMAND_TIMEOUT` seconds when set, else `default`.
pub fn limit(default: Duration) -> Duration {
    std::env::var("RUST_ANYTHING_COMMAND_TIMEOUT")
        .ok()
        .and_then(|v| v.parse::<f64>().ok())
        .and_then(seconds)
        .unwrap_or(default)
}

/// A duration from a number of seconds; `None` when the value cannot be one (not positive, not
/// finite, or too large for `Duration` — a plain conversion panics on `1e20`).
pub fn seconds(v: f64) -> Option<Duration> {
    if !v.is_finite() || v <= 0.0 {
        return None;
    }
    Duration::try_from_secs_f64(v).ok()
}

pub const TESTS: Duration = Duration::from_secs(30 * 60);
pub const BUILD: Duration = Duration::from_secs(30 * 60);
pub const QUICK: Duration = Duration::from_secs(60);

/// Runs `cmd` and captures its output (the last MiB of each stream).
pub fn output(cmd: &mut Command, default: Duration) -> Result<Output> {
    cmd.stdout(Stdio::piped()).stderr(Stdio::piped());
    let mut child = spawn(cmd)?;
    let out = drain(child.stdout.take());
    let err = drain(child.stderr.take());
    let status = wait(&mut child, cmd, limit(default))?;
    Ok(Output {
        status,
        stdout: out.join().unwrap_or_default(),
        stderr: err.join().unwrap_or_default(),
    })
}

/// Runs `cmd` with the output streams it was given.
pub fn status(cmd: &mut Command, default: Duration) -> Result<ExitStatus> {
    let mut child = spawn(cmd)?;
    wait(&mut child, cmd, limit(default))
}

/// Runs `cmd` with `input` on its stdin and the output streams it was given (an agent reading
/// its prompt).
pub fn status_with_input(cmd: &mut Command, input: &[u8], default: Duration) -> Result<ExitStatus> {
    let mut child = spawn_with(cmd, Stdio::piped())?;
    let mut stdin = child.stdin.take().context("no stdin")?;
    let input = input.to_vec();
    let writer = std::thread::spawn(move || {
        use std::io::Write as _;
        let _ = stdin.write_all(&input);
    });
    let status = wait(&mut child, cmd, limit(default));
    let _ = writer.join();
    status
}

fn spawn(cmd: &mut Command) -> Result<Child> {
    spawn_with(cmd, Stdio::null())
}

/// A failed launch of an outside command (`Command::spawn` / `Command::output`), kept as a
/// structured error so callers like `gate` can distinguish missing tools and launch failures from
/// compile errors without parsing message text.
#[derive(Debug)]
pub(crate) struct SpawnError {
    pub(crate) program: String,
    pub(crate) missing: bool,
    message: String,
    source: std::io::Error,
}

impl std::fmt::Display for SpawnError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.message)
    }
}

impl std::error::Error for SpawnError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        (!self.missing).then_some(&self.source)
    }
}

fn effective_env(cmd: &Command, key: &str) -> Option<std::ffi::OsString> {
    let mut overridden = None;
    for (k, v) in cmd.get_envs() {
        let matched = if cfg!(windows) {
            k.to_string_lossy().eq_ignore_ascii_case(key)
        } else {
            k == OsStr::new(key)
        };
        if matched {
            overridden = Some(v);
        }
    }
    match overridden {
        Some(Some(val)) => Some(val.to_os_string()),
        Some(None) => None,
        None => std::env::var_os(key),
    }
}

/// Matches `std::process::Command`'s executable check: exact file path, plus `.exe` on Windows
/// when no extension is given (Rust does not expand `PATHEXT` for `Command::spawn`).
#[cfg(windows)]
fn file_exists_as_executable(path: &Path) -> bool {
    if path.is_file() {
        return true;
    }
    if path.extension().is_none() {
        let mut with_exe = path.as_os_str().to_os_string();
        with_exe.push(".exe");
        return Path::new(&with_exe).is_file();
    }
    false
}

/// Matches `std::process::Command`'s executable check on non-Windows platforms.
#[cfg(not(windows))]
fn file_exists_as_executable(path: &Path) -> bool {
    path.is_file()
}

/// Whether `cmd`'s executable exists at its explicit path or on `cmd`'s effective `PATH`.
/// Used to distinguish a missing executable from `ENOENT` caused by a missing shebang or ELF
/// interpreter on Linux.
fn program_exists(cmd: &Command) -> bool {
    let program = cmd.get_program();
    let prog_str = program.to_string_lossy();
    let base = cmd
        .get_current_dir()
        .map(Path::to_path_buf)
        .or_else(|| std::env::current_dir().ok());
    if prog_str.contains('/') || prog_str.contains('\\') {
        let path = Path::new(program);
        return if path.is_absolute() {
            file_exists_as_executable(path)
        } else {
            base.is_some_and(|dir| file_exists_as_executable(&dir.join(path)))
        };
    }
    let Some(paths) = effective_env(cmd, "PATH") else {
        return false;
    };
    // On Unix, `Command::spawn` changes into `current_dir` before searching `PATH`; on Windows,
    // `resolve_exe` resolves relative `PATH` entries against the parent's working directory.
    let path_base = if cfg!(windows) {
        std::env::current_dir().ok()
    } else {
        base
    };
    std::env::split_paths(&paths).any(|dir| {
        let resolved = if dir.is_absolute() {
            dir
        } else if let Some(cwd) = &path_base {
            cwd.join(dir)
        } else {
            dir
        };
        file_exists_as_executable(&resolved.join(program))
    })
}

/// Turns a failed `Command::spawn` / `Command::output` into an actionable error.
/// When the program is confirmed absent (`ErrorKind::NotFound` and not found on `PATH`), names it
/// and includes `doctor`'s install command when known instead of the raw OS error.
pub(crate) fn spawn_error(cmd: &Command, what: &str, err: std::io::Error) -> anyhow::Error {
    let program = cmd.get_program().to_string_lossy().into_owned();
    if err.kind() == std::io::ErrorKind::NotFound && cmd.get_current_dir().is_none_or(Path::is_dir)
    {
        if !program_exists(cmd) {
            let message = if program.contains('/') || program.contains('\\') {
                format!("`{program}` not found; run `rust-anything doctor`")
            } else {
                match crate::doctor::missing_command(&program) {
                    Some(install) if install.contains('\n') => format!(
                        "`{program}` not found on PATH; run `rust-anything doctor` or install it:\n{install}"
                    ),
                    Some(install) => format!(
                        "`{program}` not found on PATH; install it with `{install}`, or run `rust-anything doctor`"
                    ),
                    None => format!("`{program}` not found on PATH; run `rust-anything doctor`"),
                }
            };
            return anyhow::Error::new(SpawnError {
                program,
                missing: true,
                message,
                source: err,
            });
        }
        return anyhow::Error::new(SpawnError {
            program,
            missing: false,
            message: format!(
                "running {what} (executable exists, but its interpreter or loader may be missing)"
            ),
            source: err,
        });
    }
    anyhow::Error::new(SpawnError {
        program,
        missing: false,
        message: format!("running {what}"),
        source: err,
    })
}

fn spawn_with(cmd: &mut Command, stdin: Stdio) -> Result<Child> {
    cmd.stdin(stdin);
    #[cfg(unix)]
    {
        use std::os::unix::process::CommandExt as _;
        cmd.process_group(0);
    }
    cmd.spawn()
        .map_err(|e| spawn_error(cmd, &format!("`{}`", shown(cmd)), e))
}

fn drain(stream: Option<impl Read + Send + 'static>) -> std::thread::JoinHandle<Vec<u8>> {
    std::thread::spawn(move || {
        let mut kept = Vec::new();
        let Some(mut stream) = stream else {
            return kept;
        };
        let mut buf = [0u8; 8192];
        while let Ok(n) = stream.read(&mut buf) {
            if n == 0 {
                break;
            }
            kept.extend_from_slice(&buf[..n]);
            if kept.len() > 2 * KEEP {
                kept.drain(..kept.len() - KEEP);
            }
        }
        if kept.len() > KEEP {
            kept.drain(..kept.len() - KEEP);
        }
        kept
    })
}

fn wait(child: &mut Child, cmd: &Command, limit: Duration) -> Result<ExitStatus> {
    let start = Instant::now();
    let mut pause = Duration::from_millis(5);
    loop {
        if let Some(status) = child.try_wait()? {
            return Ok(status);
        }
        if start.elapsed() > limit {
            kill(child);
            bail!(
                "`{}` was still running after {}s and was stopped (set RUST_ANYTHING_COMMAND_TIMEOUT to allow longer; a test runner in watch mode never ends, so run it once, e.g. `vitest run`)",
                shown(cmd),
                limit.as_secs()
            );
        }
        std::thread::sleep(pause);
        pause = (pause * 2).min(Duration::from_millis(100));
    }
}

fn kill(child: &mut Child) {
    #[cfg(unix)]
    {
        let _ = Command::new("/bin/kill")
            .args(["-KILL", "--", &format!("-{}", child.id())])
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status();
    }
    let _ = child.kill();
    let _ = child.wait();
}

fn shown(cmd: &Command) -> String {
    std::iter::once(cmd.get_program())
        .chain(cmd.get_args())
        .map(|a| a.to_string_lossy().into_owned())
        .collect::<Vec<_>>()
        .join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_timeout_value_that_cannot_be_a_duration_falls_back_instead_of_panicking() {
        // `Duration::from_secs_f64(1e20)` panics; an over-large value is ignored, as a bad one is.
        assert_eq!(seconds(1e20), None);
        assert_eq!(seconds(f64::INFINITY), None);
        assert_eq!(seconds(f64::NAN), None);
        assert_eq!(seconds(-1.0), None);
        assert_eq!(seconds(0.0), None);
        assert_eq!(seconds(2.5), Some(Duration::from_secs_f64(2.5)));
    }

    #[cfg(unix)]
    #[test]
    fn a_program_past_its_limit_is_stopped() {
        let start = Instant::now();
        let err = output(
            Command::new("sh").args(["-c", "sleep 30"]),
            Duration::from_millis(300),
        )
        .err()
        .map(|e| e.to_string())
        .unwrap_or_default();
        assert!(err.contains("still running"), "{err}");
        assert!(start.elapsed() < Duration::from_secs(5));
    }

    #[cfg(unix)]
    #[test]
    fn stdin_is_closed() {
        let out = output(&mut Command::new("cat"), Duration::from_secs(5)).unwrap();
        assert!(out.success());
        assert!(out.stdout.is_empty());
    }

    #[cfg(unix)]
    #[test]
    fn output_is_kept_and_capped() {
        let out = output(
            Command::new("sh").args(["-c", "yes | head -c 3000000; echo err >&2"]),
            Duration::from_secs(10),
        )
        .unwrap();
        assert_eq!(out.stdout.len(), KEEP);
        assert_eq!(out.stderr_text(), "err\n");
    }

    #[test]
    fn missing_program_names_it_and_points_to_doctor() {
        let program = "rust-anything-nonexistent-program-498";
        let err = output(&mut Command::new(program), Duration::from_secs(5))
            .err()
            .expect("nonexistent program must fail");
        let msg = format!("{err:#}");
        assert!(
            msg.contains(&format!("`{program}` not found on PATH")),
            "{msg}"
        );
        assert!(msg.contains("rust-anything doctor"), "{msg}");
        assert!(!msg.contains("os error"), "{msg}");
    }

    #[test]
    fn missing_cargo_includes_doctor_install_command() {
        let mut cmd = Command::new("cargo");
        cmd.env_remove("PATH");
        let err = spawn_error(
            &cmd,
            "cargo build",
            std::io::Error::new(std::io::ErrorKind::NotFound, "not found"),
        );
        let spawn = err.downcast_ref::<SpawnError>().unwrap();
        assert!(spawn.missing);
        assert_eq!(spawn.program, "cargo");
        let msg = format!("{err:#}");
        let install = crate::doctor::missing_command("cargo").unwrap();
        assert!(msg.contains("`cargo` not found on PATH"), "{msg}");
        assert!(msg.contains(&install), "{msg}");
        assert!(msg.contains("rust-anything doctor"), "{msg}");
    }

    #[test]
    fn existing_executable_with_not_found_keeps_launch_context() {
        let dir = std::env::temp_dir().join(format!("ra-proc-exists-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        let wrapper = dir.join("fake-cargo");
        std::fs::write(&wrapper, "#!/missing/interpreter\n").unwrap();

        let mut on_path = Command::new("fake-cargo");
        on_path.env("PATH", &dir);
        let err = spawn_error(
            &on_path,
            "cargo build",
            std::io::Error::new(std::io::ErrorKind::NotFound, "no interpreter"),
        );
        let spawn = err.downcast_ref::<SpawnError>().unwrap();
        assert!(!spawn.missing);
        let msg = format!("{err:#}");
        assert!(msg.starts_with("running cargo build"), "{msg}");
        assert!(msg.contains("interpreter"), "{msg}");
        assert!(msg.contains("no interpreter"), "{msg}");
        assert!(!msg.contains("not found on PATH"), "{msg}");

        let explicit = Command::new(&wrapper);
        let explicit_err = spawn_error(
            &explicit,
            "fake-cargo",
            std::io::Error::new(std::io::ErrorKind::NotFound, "no interpreter"),
        );
        assert!(!explicit_err.downcast_ref::<SpawnError>().unwrap().missing);

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn relative_path_entry_resolves_against_child_cwd() {
        let dir = std::env::temp_dir().join(format!("ra-proc-relpath-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        let bin = dir.join("bin");
        std::fs::create_dir_all(&bin).unwrap();
        let name = if cfg!(windows) {
            "fake-rel-tool.exe"
        } else {
            "fake-rel-tool"
        };
        std::fs::write(bin.join(name), "").unwrap();

        let mut cmd = Command::new("fake-rel-tool");
        cmd.current_dir(&dir).env("PATH", "bin");
        let err = spawn_error(
            &cmd,
            "fake-rel-tool",
            std::io::Error::new(std::io::ErrorKind::NotFound, "no interpreter"),
        );
        if cfg!(windows) {
            assert!(
                err.downcast_ref::<SpawnError>().unwrap().missing,
                "on Windows, relative PATH entries resolve against the parent cwd, not child cwd"
            );
        } else {
            assert!(
                !err.downcast_ref::<SpawnError>().unwrap().missing,
                "on Unix, relative PATH entries resolve against child cwd"
            );
        }

        #[cfg(windows)]
        {
            std::fs::write(bin.join("only-cmd.cmd"), "@echo off\r\n").unwrap();
            let mut cmd_only = Command::new("only-cmd");
            cmd_only.env("PATH", &bin);
            let err = spawn_error(
                &cmd_only,
                "only-cmd",
                std::io::Error::new(std::io::ErrorKind::NotFound, "not found"),
            );
            assert!(
                err.downcast_ref::<SpawnError>().unwrap().missing,
                "bare extensionless lookup on Windows should not match .cmd via PATHEXT"
            );
        }

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn missing_cwd_or_other_spawn_errors_keep_running_context() {
        let mut bad_cwd = Command::new("cargo");
        bad_cwd.env_remove("PATH");
        bad_cwd.current_dir(std::env::temp_dir().join("rust-anything-no-such-dir-498"));
        let err = spawn_error(
            &bad_cwd,
            "cargo build",
            std::io::Error::new(std::io::ErrorKind::NotFound, "missing cwd"),
        );
        assert!(!err.downcast_ref::<SpawnError>().unwrap().missing);
        let msg = format!("{err:#}");
        assert!(msg.starts_with("running cargo build"), "{msg}");
        assert!(!msg.contains("not found on PATH"), "{msg}");

        let cmd = Command::new("cargo");
        let perm = spawn_error(
            &cmd,
            "cargo build",
            std::io::Error::new(std::io::ErrorKind::PermissionDenied, "denied"),
        );
        assert!(!perm.downcast_ref::<SpawnError>().unwrap().missing);
        let perm_msg = format!("{perm:#}");
        assert!(perm_msg.starts_with("running cargo build"), "{perm_msg}");
    }
}
