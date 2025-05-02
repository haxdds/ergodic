use axum::{
    routing::{post, get},
    extract::Json,
    Router,
};
use serde::Deserialize;
use crossbeam_channel as channel;
use channel::Sender;
use crate::core::{Order, Side, now_ns};
use crate::engine::Msg;

/// Represents a new order request received via the API
#[derive(Deserialize)]
pub struct NewOrder { id: u64, side: String, price: i64, qty: u64 }

/// Creates and returns the API router with configured endpoints
pub fn router(tx: Sender<Msg>) -> Router {
    Router::new()
        // POST endpoint for submitting new orders
        .route("/order", post({
            let tx = tx.clone();
            move |Json(o): Json<NewOrder>| async move {
                // Convert the side string to Side enum
                // Accepts "bid"/"b" for bids, anything else is treated as ask
                let side = matches!(o.side.to_lowercase().as_str(), "bid" | "b")
                    .then_some(Side::Bid)
                    .unwrap_or(Side::Ask);

                // Create and send the order to the trading engine
                let order = Order { id: o.id, side, price: o.price, qty: o.qty, ts: now_ns() };
                let _ = tx.send(Msg::NewOrder(order));    // ignore send‑error on shutdown
                "accepted"
            }
        }))
        // GET endpoint for retrieving current best bid/ask quote
        .route("/quote", get({
            move || async move {
                // Create a one-shot channel for receiving the quote response
                let (reply_tx, reply_rx) = crossbeam::channel::bounded(1);
                // Send quote request to trading engine
                let _ = tx.send(Msg::QuoteReq(reply_tx));
                // Wait for and format the response
                match reply_rx.recv() {
                    Ok(Some((b, a))) => format!("{{\"bid\":{},\"ask\":{}}}", b, a), // Return JSON with bid/ask
                    _                => "NA".into(), // Return NA if no quote available or error
                }
            }
        }))
}