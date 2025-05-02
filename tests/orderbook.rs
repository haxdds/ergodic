use ergodic::core::{OrderBook, Order, Side, now_ns};

#[cfg(test)]
mod orderbook_tests {
    use super::*;

    #[test]
    fn no_trade_single_order() {
        let mut ob = OrderBook::default();
        let ask = Order { id: 1, side: Side::Ask, price: 100, qty: 10, ts: now_ns() };
        let trades = ob.submit(ask);
        assert!(trades.is_empty(), "no trades with only one side");
    }

    #[test]
    fn partial_fill_and_remaining_levels() {
        let mut ob = OrderBook::default();
        ob.submit(Order { id: 2, side: Side::Ask, price: 50, qty: 10, ts: now_ns() });
        let trades = ob.submit(Order { id: 3, side: Side::Bid, price: 55, qty: 6, ts: now_ns() });
        assert_eq!(trades.len(), 1);
        assert_eq!(trades[0].price, 50);
        assert_eq!(trades[0].qty, 6);
        let Some((_, ask)) = ob.best_bid_ask() else { panic!("expected ask") };
        assert_eq!(ask, 50);
    }

    #[test]
    fn full_match_clears_level() {
        let mut ob = OrderBook::default();
        ob.submit(Order { id: 4, side: Side::Ask, price: 75, qty: 5, ts: now_ns() });
        ob.submit(Order { id: 5, side: Side::Bid, price: 80, qty: 5, ts: now_ns() });
        assert!(ob.best_bid_ask().is_none());
    }
}