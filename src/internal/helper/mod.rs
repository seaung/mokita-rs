use tokio::signal::unix::{SignalKind, signal};

pub async fn shutdown_with_signal() {
    let mut sigint = signal(SignalKind::interrupt()).unwrap();
    let mut sighup = signal(SignalKind::hangup()).unwrap();
    let mut sigquit = signal(SignalKind::quit()).unwrap();
    let mut sigterm = signal(SignalKind::terminate()).unwrap();

    tokio::select! {
        _ = sigint.recv() => {
            println!("recv interrupt signal...");
        }

        _ = sighup.recv() => {
            println!("recv hangup signal...");
        }

        _ = sigquit.recv() => {
            println!("recv quit signal...");
        }

        _ = sigterm.recv() => {
            println!("recv terminate signal...");
        }
    }
}
