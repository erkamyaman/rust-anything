mod units;

use std::net::SocketAddr;
use std::process::ExitCode;

use app_rs::port::cli;
use axum::serve::ListenerExt;

#[tokio::main]
async fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let checked = cli::parse(
        &args,
        std::env::var("PORT").ok(),
        std::env::var("HOST").ok(),
    )
    .and_then(|options| cli::check_file(&options).map(|()| options));
    let options = match checked {
        Ok(options) => options,
        Err(exit) => {
            print!("{}", exit.stdout);
            eprint!("{}", exit.stderr);
            return ExitCode::from(exit.code as u8);
        }
    };
    let Some(port) = options.port else {
        eprintln!(
            "RangeError [ERR_SOCKET_BAD_PORT]: options.port should be >= 0 and < 65536. Received {}.",
            options.port_text
        );
        return ExitCode::from(1);
    };
    let banner = units::app::banner();
    let listener =
        match tokio::net::TcpListener::bind(SocketAddr::from(([0u16, 0, 0, 0, 0, 0, 0, 0], port)))
            .await
        {
            Ok(listener) => listener,
            Err(_) => {
                match tokio::net::TcpListener::bind(SocketAddr::from(([0, 0, 0, 0], port))).await {
                    Ok(listener) => listener,
                    Err(e) => {
                        eprintln!("Error: listen {e} :::{port}");
                        return ExitCode::from(1);
                    }
                }
            }
        };
    let listener = listener.tap_io(|tcp| {
        let _ = tcp.set_nodelay(true);
    });
    print!("{banner}");
    if axum::serve(listener, units::router())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .is_err()
    {
        return ExitCode::from(1);
    }
    ExitCode::SUCCESS
}

/// Ctrl-C or SIGTERM: stop taking requests and finish the open ones.
pub async fn shutdown_signal() {
    #[cfg(unix)]
    {
        let mut term = tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("a SIGTERM handler");
        tokio::select! {
            _ = tokio::signal::ctrl_c() => {}
            _ = term.recv() => {}
        }
    }
    #[cfg(not(unix))]
    {
        let _ = tokio::signal::ctrl_c().await;
    }
}
