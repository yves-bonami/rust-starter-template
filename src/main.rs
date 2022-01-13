use tokio::{select, signal, time};
use tracing::info;

#[tokio::main]
async fn main() {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "debug")
    }
    tracing_subscriber::fmt::init();

    info!("application starting");

    // Run application and await end or shutdown signal
    select! {
        _ = shutdown_signal() => {},
        _ = application_run() => {},
    }

    info!("application stopped");
}

async fn application_run() {
    let mut interval = time::interval(time::Duration::from_secs(1));
    loop {
        info!("application running");
        interval.tick().await;
    }
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}
