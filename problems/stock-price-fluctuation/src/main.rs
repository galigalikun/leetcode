struct StockPrice {
    prices: Vec<(i32, i32)>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl StockPrice {

    fn new() -> Self {
        StockPrice {
            prices: Vec::new(),
        }
    }

    fn update(&mut self, timestamp: i32, price: i32) {
        self.prices.push((timestamp, price));
    }

    fn current(&self) -> i32 {
        match self.prices.last() {
            Some((_, price)) => *price,
            None => 0,
        }
    }

    fn maximum(&self) -> i32 {
        match self.prices.iter().max_by_key(|(_, price)| price) {
            Some((_, price)) => *price,
            None => 0,
        }
    }

    fn minimum(&self) -> i32 {
        match self.prices.iter().min_by_key(|(_, price)| price) {
            Some((_, price)) => *price,
            None => 0,
        }
    }
}

/**
 * Your StockPrice object will be instantiated and called as such:
 * let obj = StockPrice::new();
 * obj.update(timestamp, price);
 * let ret_2: i32 = obj.current();
 * let ret_3: i32 = obj.maximum();
 * let ret_4: i32 = obj.minimum();
 */
fn main() {
    let mut obj = StockPrice::new();
    obj.update(1, 10);
    obj.update(2, 5);
    assert_eq!(obj.current(), 5);
    assert_eq!(obj.maximum(), 10);
    obj.update(1, 3);
    assert_eq!(obj.maximum(), 5);
    obj.update(4, 2);
    assert_eq!(obj.minimum(), 2);
}
