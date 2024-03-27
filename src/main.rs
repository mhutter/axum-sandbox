use axum::Router;
use axum_sandbox::experiments::session;

fn main() {
    // Initialize tracing/logger
    tracing_subscriber::fmt::init();

    // Doing this here manually instead of using the #[tokio::main] macro saves us another handful
    // of dependencies
    let app = async {
        if let Err(err) = run().await {
            tracing::error!("ERROR: {err}");
            std::process::exit(1);
        }
    };

    // Create the async runtime and start the application
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("Failed building the Runtime")
        .block_on(app);
}

// "actual" main entrypoint of the application
async fn run() -> Result<(), String> {
    let app = Router::new().merge(session::routes());

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
    Ok(())
}
