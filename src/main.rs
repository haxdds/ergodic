use tokio::net::TcpListener;
use crossbeam_channel as channel; 

use ergodic::{api, engine, core};

/// Main entry point for the trading engine application
/// Configures a multi-threaded runtime with 4 worker threads
#[tokio::main(flavor = "multi_thread", worker_threads = 4)]
async fn main() {
    // Create unbounded channels for:
    // - ing_tx/rx: Sending orders/requests to the trading engine
    // - trade_tx/rx: Publishing executed trades from the engine
    let (ing_tx, ing_rx)      = channel::unbounded();
    let (trade_tx, _trade_rx) = channel::unbounded::<core::Trade>();

    // Spawn the trading engine on a dedicated thread
    // The engine processes orders and quote requests received through ing_rx
    // and publishes trades through trade_tx
    std::thread::spawn(move || engine::run(ing_rx, trade_tx));

    // Create the HTTP API router that will handle order submissions
    // and quote requests, passing them to the trading engine via ing_tx
    let app = api::router(ing_tx);

    // Bind the HTTP server to all interfaces on port 8080
    let listener = TcpListener::bind("0.0.0.0:8080")
        .await
        .expect("bind failed");

    // Start serving HTTP requests
    // The server will run until an error occurs or the process is terminated
    axum::serve(listener, app)
        .await
        .expect("server error");
}