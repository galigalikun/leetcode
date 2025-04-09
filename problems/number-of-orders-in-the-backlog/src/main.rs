fn main() {
    assert_eq!(
        Solution::get_number_of_backlog_orders(vec![
            vec![10, 5, 0],
            vec![15, 2, 1],
            vec![25, 1, 1],
            vec![30, 4, 0]
        ]),
        6
    );
    assert_eq!(
        Solution::get_number_of_backlog_orders(vec![
            vec![7, 1000000000, 0],
            vec![15, 2, 1],
            vec![25, 1, 1],
            vec![30, 4, 0]
        ]),
        3
    );
}

struct Solution;
impl Solution {
    pub fn get_number_of_backlog_orders(orders: Vec<Vec<i32>>) -> i32 {
        use std::collections::BinaryHeap;
        let mut buy = BinaryHeap::new();
        let mut sell = BinaryHeap::new();
        for order in orders {
            let price = order[0];
            let amount = order[1];
            let order_type = order[2];
            if order_type == 0 {
                buy.push((price, amount));
            } else {
                sell.push((-price, amount));
            }
        }
        let mut backlog = 0;
        while !buy.is_empty() && !sell.is_empty() {
            let buy_order = buy.peek().unwrap();
            let sell_order = sell.peek().unwrap();
            if buy_order.0 >= -sell_order.0 {
                let amount = buy_order.1.min(-sell_order.1);
                backlog += amount;
                if amount == buy_order.1 {
                    buy.pop();
                } else {
                    buy.push((buy_order.0, buy_order.1 - amount));
                }
                if amount == -sell_order.1 {
                    sell.pop();
                } else {
                    sell.push((-sell_order.0, -sell_order.1 - amount));
                }
            } else {
                break;
            }
        }
        while !buy.is_empty() {
            let buy_order = buy.pop().unwrap();
            backlog += buy_order.1;
        }
        while !sell.is_empty() {
            let sell_order = sell.pop().unwrap();
            backlog += -sell_order.1;
        }
        backlog %= 1_000_000_007;
        backlog
    }
}
