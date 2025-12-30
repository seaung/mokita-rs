use tokio::signal::unix::{SignalKind, signal};

async fn shutdown_with_signal() {
    let mut sigint = signal(SignalKind::interrupt()).unwrap();
    let mut sighup = signal(SignalKind::hangup()).unwrap();
    let mut sigquit = signal(SignalKind::quit()).unwrap();
    let mut sigterm = signal(SignalKind::terminate()).unwrap();

    tokio::select! {
        _ = sigint.recv() => {
            println!("recv SIGINT signal. shutdown server...");
        }
        _ = sighup.recv() => {
            println!("recv SIGHUP signal. shutdown server...");
        }
        _ = sigquit.recv() => {
            println!("recv SIGQUIT signal. shutdown server...");
        }
        _ = sigterm.recv() => {
            println!("recv SIGTERM signal. shutdown server...");
        }
    }
}
