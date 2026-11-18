//! `bin.ts`'s command line: `node:util` `parseArgs` with its options, `--help`, `--version`,
//! the checks on the file, and what it prints when the server starts.

use std::io::IsTerminal;
use std::path::PathBuf;

pub const VERSION: &str = "1.0.0-beta.15";

const HELP: &str = "Usage: json-server [options] <file>

Options:
  -p, --port <port>  Port (default: 3000)
  -h, --host <host>  Host (default: localhost)
  -s, --static <dir> Static files directory (multiple allowed)
  --help             Show this message
  --version          Show version number
";

/// What the command line asks for.
#[derive(Debug, Clone, PartialEq)]
pub struct Options {
    pub file: PathBuf,
    /// `parseInt(port)`: `None` when it is not a number.
    pub port: Option<u16>,
    pub port_text: String,
    pub host: String,
    pub static_dirs: Vec<PathBuf>,
}

/// What to do instead of serving: print and exit with this code.
#[derive(Debug, Clone, PartialEq)]
pub struct Exit {
    pub stdout: String,
    pub stderr: String,
    pub code: i32,
}

fn color(code: (u8, u8), text: &str) -> String {
    if std::io::stdout().is_terminal() {
        format!("\x1b[{}m{text}\x1b[{}m", code.0, code.1)
    } else {
        text.to_string()
    }
}

pub fn bold(t: &str) -> String {
    color((1, 22), t)
}
pub fn gray(t: &str) -> String {
    color((90, 39), t)
}
pub fn red(t: &str) -> String {
    color((31, 39), t)
}
pub fn yellow(t: &str) -> String {
    color((33, 39), t)
}
pub fn blue(t: &str) -> String {
    color((34, 39), t)
}
pub fn magenta(t: &str) -> String {
    color((35, 39), t)
}

fn unknown(option: &str) -> Exit {
    Exit {
        stdout: format!("{}\n{HELP}\n", red(&format!("Unknown option '{option}'"))),
        stderr: String::new(),
        code: 1,
    }
}

fn missing(option: &str) -> Exit {
    Exit {
        stdout: String::new(),
        stderr: format!(
            "TypeError [ERR_PARSE_ARGS_INVALID_OPTION_VALUE]: Option '{option}' argument missing\n"
        ),
        code: 1,
    }
}

/// Parses the arguments after the program name, with `PORT` and `HOST` from the environment.
pub fn parse(
    args: &[String],
    env_port: Option<String>,
    env_host: Option<String>,
) -> Result<Options, Exit> {
    let mut port = env_port.unwrap_or_else(|| "3000".into());
    let mut host = env_host.unwrap_or_else(|| "localhost".into());
    let mut statics = Vec::new();
    let (mut help, mut version, mut watch) = (false, false, false);
    let mut positionals: Vec<String> = Vec::new();
    let mut i = 0;
    while i < args.len() {
        let arg = &args[i];
        i += 1;
        if arg == "--" {
            positionals.extend(args[i..].iter().cloned());
            break;
        }
        let (name, inline) = match arg.strip_prefix("--") {
            Some(long) => match long.split_once('=') {
                Some((n, v)) => (n.to_string(), Some(v.to_string())),
                None => (long.to_string(), None),
            },
            None if arg.starts_with('-') && arg.len() > 1 => {
                let short = &arg[1..2];
                let long = match short {
                    "p" => "port",
                    "h" => "host",
                    "s" => "static",
                    "w" => "watch",
                    _ => return Err(unknown(&format!("-{short}"))),
                };
                let rest = &arg[1 + short.len()..];
                (
                    long.to_string(),
                    (!rest.is_empty()).then(|| rest.to_string()),
                )
            }
            None => {
                positionals.push(arg.clone());
                continue;
            }
        };
        let mut value = |label: &str| -> Result<String, Exit> {
            if let Some(v) = &inline {
                return Ok(v.clone());
            }
            match args.get(i) {
                Some(v) if !v.starts_with('-') || v.len() == 1 => {
                    i += 1;
                    Ok(v.clone())
                }
                _ => Err(missing(label)),
            }
        };
        match name.as_str() {
            "port" => port = value("-p, --port <value>")?,
            "host" => host = value("-h, --host <value>")?,
            "static" => statics.push(PathBuf::from(value("-s, --static <value>")?)),
            "help" => help = true,
            "version" => version = true,
            "watch" => watch = true,
            other => return Err(unknown(&format!("--{other}"))),
        }
    }
    if version {
        return Err(Exit {
            stdout: format!("{VERSION}\n"),
            stderr: String::new(),
            code: 0,
        });
    }
    let mut stdout = String::new();
    if watch {
        stdout += &yellow(
            "--watch/-w can be omitted, JSON Server 1+ watches for file changes by default",
        );
        stdout.push('\n');
    }
    if help || positionals.is_empty() {
        stdout += HELP;
        stdout.push('\n');
        return Err(Exit {
            stdout,
            stderr: String::new(),
            code: 0,
        });
    }
    if !stdout.is_empty() {
        print!("{stdout}");
    }
    let parsed = json_server_js::number::parse_int(&port);
    Ok(Options {
        file: PathBuf::from(&positionals[0]),
        port: (parsed.is_finite() && (0.0..=65535.0).contains(&parsed)).then_some(parsed as u16),
        port_text: json_server_js::number::number_to_string(parsed),
        host,
        static_dirs: statics,
    })
}

/// The checks `bin.ts` makes on the file before it opens it.
pub fn check_file(options: &Options) -> Result<(), Exit> {
    let file = &options.file;
    if !file.exists() {
        return Err(Exit {
            stdout: format!("{}\n", red(&format!("File {} not found", file.display()))),
            stderr: String::new(),
            code: 1,
        });
    }
    if file.extension().is_some_and(|e| e == "json5") {
        return Err(Exit {
            stdout: String::new(),
            stderr: format!(
                "{}: JSON5 files are not supported by this port; convert it to JSON\n",
                file.display()
            ),
            code: 1,
        });
    }
    if let Ok(text) = std::fs::read_to_string(file)
        && json_server_js::number::trim(&text).is_empty()
    {
        let _ = std::fs::write(file, "{}");
    }
    Ok(())
}

/// `logRoutes`.
pub fn routes(options: &Options, keys: &[&String]) -> String {
    let mut out = format!("{}\n", bold("Endpoints:"));
    if keys.is_empty() {
        out += &gray(&format!(
            "No endpoints found, try adding some data to {}",
            options.file.display()
        ));
        out.push('\n');
        return out;
    }
    let base = format!("http://{}:{}/", options.host, options.port_text);
    let lines: Vec<String> = keys
        .iter()
        .map(|k| format!("{}{}", gray(&base), blue(k)))
        .collect();
    out += &lines.join("\n");
    out.push('\n');
    out
}

/// What `bin.ts` prints once the server listens.
pub fn banner(options: &Options, keys: &[&String]) -> String {
    const KAOMOJIS: [&str; 4] = ["♡⸜(˶˃ ᵕ ˂˶)⸝♡", "♡( ◡‿◡ )", "( ˶ˆ ᗜ ˆ˵ )", "(˶ᵔ ᵕ ᵔ˶)"];
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_or(0, |d| d.subsec_nanos());
    let kaomoji = KAOMOJIS[(nanos as usize) % KAOMOJIS.len()];
    let port = &options.port_text;
    let lines = [
        bold(&format!("JSON Server started on PORT :{port}")),
        gray("Press CTRL-C to stop"),
        gray(&format!("Watching {}...", options.file.display())),
        String::new(),
        magenta(kaomoji),
        String::new(),
        bold("Index:"),
        gray(&format!("http://localhost:{port}/")),
        String::new(),
        bold("Static files:"),
        gray("Serving ./public directory if it exists"),
        String::new(),
    ];
    format!("{}\n{}", lines.join("\n"), routes(options, keys))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn args(s: &str) -> Vec<String> {
        s.split_whitespace().map(str::to_string).collect()
    }

    #[test]
    fn the_command_line_parses_as_bin_ts() {
        let o = parse(&args("db.json -p 4000 -s a --static=b"), None, None).unwrap();
        assert_eq!(o.port, Some(4000));
        assert_eq!(o.static_dirs, [PathBuf::from("a"), PathBuf::from("b")]);
        assert_eq!(
            parse(&args("db.json"), Some("5000".into()), None)
                .unwrap()
                .port,
            Some(5000)
        );
        assert_eq!(
            parse(&args("--version"), None, None).unwrap_err().stdout,
            "1.0.0-beta.15\n"
        );
        assert_eq!(parse(&args(""), None, None).unwrap_err().code, 0);
        let bad = parse(&args("--nope db.json"), None, None).unwrap_err();
        assert!(bad.stdout.starts_with("Unknown option '--nope'") && bad.code == 1);
    }
}
