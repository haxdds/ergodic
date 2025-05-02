use std::{
    collections::{BTreeMap, VecDeque},
    time::{SystemTime, UNIX_EPOCH},
};

/// Represents the side of an order - either a bid (buy) or ask (sell)
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Side { Bid, Ask }

/// Represents a single order in the order book
#[derive(Clone)]
pub struct Order {
    pub id:    u64,    // Unique identifier for the order
    pub side:  Side,   // Whether this is a bid or ask
    pub price: i64,    // Price in ticks
    pub qty:   u64,    // Quantity remaining to be filled
    pub ts:    u128,   // Timestamp in nanoseconds since epoch
}

/// The main order book structure that maintains sorted bids and asks
#[derive(Default)]
pub struct OrderBook {
    // Bids sorted by price in descending order (highest first)
    bids: BTreeMap<i64, VecDeque<Order>>, 
    // Asks sorted by price in ascending order (lowest first)
    asks: BTreeMap<i64, VecDeque<Order>>,
}

/// Represents a completed trade between two orders
#[derive(Clone)]
pub struct Trade { pub price: i64, pub qty: u64 }

impl OrderBook {
    /// Submit a new order to the book and return any trades that result
    /// 
    /// This implements a price-time priority matching algorithm:
    /// - For bids: matches against lowest-priced asks first
    /// - For asks: matches against highest-priced bids first
    /// - Within a price level, matches oldest orders first (FIFO)
    pub fn submit(&mut self, mut o: Order) -> Vec<Trade> {
        // Get reference to opposing side's orders (asks for bids, bids for asks)
        let contra = if o.side == Side::Bid { &mut self.asks } else { &mut self.bids };
        let mut trades = Vec::new();

        // Continue matching while we still have quantity to fill
        while o.qty > 0 {
            // Find best price on opposing side
            let best = match o.side {
                Side::Bid => contra.keys().next().cloned(),        // Lowest ask
                Side::Ask => contra.keys().rev().next().cloned(),  // Highest bid
            };
            let p = match best { Some(p) => p, None => break };

            // Check if price crosses (matches are possible)
            let matchable = (o.side == Side::Bid && o.price >= p)
                         || (o.side == Side::Ask && o.price <= p);
            if !matchable { break; }

            // Match against the oldest order at this price level
            let q = contra.get_mut(&p).unwrap();
            let head = q.front_mut().unwrap();
            let filled = o.qty.min(head.qty);
            head.qty -= filled;
            o.qty    -= filled;
            trades.push(Trade { price: p, qty: filled });

            // Clean up fully filled orders
            if head.qty == 0 { q.pop_front(); }
            if q.is_empty()  { contra.remove(&p); }
        }

        // If order was not fully filled, add remainder to book
        if o.qty > 0 {
            let same = if o.side == Side::Bid { &mut self.bids } else { &mut self.asks };
            same.entry(o.price).or_default().push_back(o);
        }
        trades
    }

    /// Returns the current best bid and ask prices, if they exist
    pub fn best_bid_ask(&self) -> Option<(i64,i64)> {
        Some((*self.bids.keys().next_back()?, *self.asks.keys().next()?))
    }
}

/// Returns current time in nanoseconds since Unix epoch
pub fn now_ns() -> u128 {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos()
}