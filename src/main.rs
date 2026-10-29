use std::collections::{BTreeMap, HashMap}; //I think we will need only these 2 for sorting and searching

#[derive(Debug, Clone, Copy, PartialEq)] //Gives me abilities attached to my order
enum Side {
    Bid,
    Ask,
}

#[derive(Debug, Clone)] //Gives me abilities attached to my order
struct Order { //Prices and quantities are always whole number 
    id: u64,
    side: Side,
    price: u64,
    qty: u64,
}

#[derive(Debug, PartialEq)]
enum BookError { //Always better rather than strings we can add more errors later 
    DuplicateId,
    ZeroQuantity,
    UnknownOrder,
}

struct OrderBook {
    bids: BTreeMap<u64, u64>, // price and total quantity
    asks: BTreeMap<u64, u64>,
    orders: HashMap<u64, Order>, // id and order, so we can cancel
}

impl OrderBook {
    fn new() -> Self {
        OrderBook {
            bids: BTreeMap::new(),
            asks: BTreeMap::new(),
            orders: HashMap::new(),
        }
    }

    fn add(&mut self, order: Order) -> Result<(), BookError> {
        if order.qty == 0 {
            return Err(BookError::ZeroQuantity);
        }
        if self.orders.contains_key(&order.id) {
            return Err(BookError::DuplicateId);
        }
        let levels = match order.side {
            Side::Bid => &mut self.bids,
            Side::Ask => &mut self.asks,
        };
        *levels.entry(order.price).or_insert(0) += order.qty;
        self.orders.insert(order.id, order);
        Ok(())
    }

    fn cancel(&mut self, id: u64) -> Result<Order, BookError> {
        let order = self.orders.remove(&id).ok_or(BookError::UnknownOrder)?;
        let levels = match order.side {
            Side::Bid => &mut self.bids,
            Side::Ask => &mut self.asks,
        };
        let level_qty = levels.get_mut(&order.price).unwrap();
        *level_qty -= order.qty;
        if *level_qty == 0 {
            levels.remove(&order.price);
        }
        Ok(order)
    }

    fn best_bid(&self) -> Option<u64> {
        self.bids.keys().next_back().copied()
    }

    fn best_ask(&self) -> Option<u64> {
        self.asks.keys().next().copied()
    }

    fn spread(&self) -> Option<u64> {
        let bid = self.best_bid()?;
        let ask = self.best_ask()?;
        Some(ask - bid)
    }
}

fn main() {
    let mut book = OrderBook::new(); //mut bcuz add and cancel modify the assigned memory for new orders

    book.add(Order { id: 1, side: Side::Bid, price: 100, qty: 5 }).unwrap(); //J Unrap isjust for the Demo otherwise proper match statments for all 
    book.add(Order { id: 2, side: Side::Bid, price: 101, qty: 3 }).unwrap();
    book.add(Order { id: 3, side: Side::Ask, price: 104, qty: 2 }).unwrap();
    book.add(Order { id: 4, side: Side::Ask, price: 103, qty: 7 }).unwrap();

    println!("best bid: {:?}", book.best_bid());
    println!("best ask: {:?}", book.best_ask());
    println!("spread:   {:?}", book.spread());

    book.cancel(2).unwrap();
    println!("after cancelling order 2, best bid: {:?}", book.best_bid());

    match book.cancel(99) {
        Ok(order) => println!("cancelled {:?}", order),
        Err(e) => println!("cancel failed: {:?}", e),
    }
}

//few unit tests
#[cfg(test)]
mod tests {
    use super::*;

    //empty book case ( no best bid and spread)
    #[test]
    fn test_empty_book() {
        let book = OrderBook::new();
        assert_eq!(book.best_bid(), None);
        assert_eq!(book.spread(), None);
    }

    //all helper methods are read correctly or not 
    #[test]
    fn test_best_prices() {
        let mut book = OrderBook::new();
        book.add(Order { id: 1, side: Side::Bid, price: 100, qty: 5 }).unwrap();
        book.add(Order { id: 2, side: Side::Ask, price: 103, qty: 2 }).unwrap();
        assert_eq!(book.best_bid(), Some(100));
        assert_eq!(book.best_ask(), Some(103));
        assert_eq!(book.spread(), Some(3));
    }

    // cancelling the top bid should remove its current level and the 2nd in line price will become best
    #[test]
    fn test_cancel() {
        let mut book = OrderBook::new();
        book.add(Order { id: 1, side: Side::Bid, price: 100, qty: 5 }).unwrap();
        book.add(Order { id: 2, side: Side::Bid, price: 101, qty: 3 }).unwrap();
        book.cancel(2).unwrap();
        assert_eq!(book.best_bid(), Some(100));
    }

    // duplicate ID,zero quantity and unknown cancel all should be rejected
    #[test]
    fn test_errors() {
        let mut book = OrderBook::new();
        book.add(Order { id: 1, side: Side::Bid, price: 100, qty: 5 }).unwrap();
        let dup = book.add(Order { id: 1, side: Side::Ask, price: 105, qty: 1 });
        assert_eq!(dup, Err(BookError::DuplicateId));
        let zero = book.add(Order { id: 2, side: Side::Ask, price: 105, qty: 0 });
        assert_eq!(zero, Err(BookError::ZeroQuantity));
        assert_eq!(book.cancel(99).unwrap_err(), BookError::UnknownOrder);
    }
}