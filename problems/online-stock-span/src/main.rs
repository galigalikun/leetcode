struct StockSpanner {
    stack: Vec<(i32, i32)>,
}

impl StockSpanner {
    fn new() -> Self {
        StockSpanner { stack: Vec::new() }
    }

    fn next(&mut self, price: i32) -> i32 {
        let mut span = 1;

        while let Some(&(top_price, top_span)) = self.stack.last() {
            if top_price <= price {
                span += top_span;
                self.stack.pop();
            } else {
                break;
            }
        }

        self.stack.push((price, span));
        span
    }
}

fn main() {
    let mut obj = StockSpanner::new();
    assert_eq!(obj.next(100), 1);
    assert_eq!(obj.next(80), 1);
    assert_eq!(obj.next(60), 1);
    assert_eq!(obj.next(70), 2);
    assert_eq!(obj.next(60), 1);
    assert_eq!(obj.next(75), 4);
    assert_eq!(obj.next(85), 6);
}
