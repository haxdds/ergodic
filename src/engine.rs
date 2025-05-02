use crossbeam_channel as channel;
use channel::{Receiver, Sender};
use crate::core::{OrderBook, Order, Trade};

/// Message types that can be processed by the trading engine
pub enum Msg {
    /// A new order to be submitted to the order book
    NewOrder(Order),
    /// A request for the current best bid/ask quote, with a channel to send the response
    /// Response format is Option<(bid_price, ask_price)>
    QuoteReq(Sender<Option<(i64,i64)>>),
}

/// Main trading engine loop that processes incoming messages and manages the order book
///
/// # Arguments
/// * `rx` - Receiver channel for incoming messages (orders and quote requests)
/// * `trades_tx` - Sender channel to publish executed trades
///
/// The engine runs continuously until the input channel is closed, processing:
/// - New orders: Submits them to the order book and publishes any resulting trades
/// - Quote requests: Returns the current best bid/ask prices
pub fn run(rx: Receiver<Msg>, trades_tx: Sender<Trade>) {
    // Initialize an empty order book
    let mut book = OrderBook::default();

    // Process messages until the channel is closed
    while let Ok(msg) = rx.recv() {
        match msg {
            Msg::NewOrder(o) => {
                // Submit order to book and process any resulting trades
                for t in book.submit(o) {
                    let _ = trades_tx.try_send(t); // drop on back‑pressure
                }
            }
            Msg::QuoteReq(resp) => {
                // Retrieve and send current best bid/ask prices
                let _ = resp.send(book.best_bid_ask());
            }
        }
    }
}