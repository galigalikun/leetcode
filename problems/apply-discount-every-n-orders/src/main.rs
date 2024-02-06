struct Cashier {
    n: i32,
    discount: i32,
    products: Vec<i32>,
    prices: Vec<i32>,
    count: i32,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Cashier {
    fn new(n: i32, discount: i32, products: Vec<i32>, prices: Vec<i32>) -> Self {
        Cashier {
            n: n,
            discount: discount,
            products: products,
            prices: prices,
            count: 0,
        }
    }

    fn get_bill(&mut self, product: Vec<i32>, amount: Vec<i32>) -> f64 {
        let mut total = 0.0;
        for i in 0..product.len() {
            total += amount[i] as f64;
        }
        self.count += 1;
        if self.count == self.n {
            total = total - (self.discount as f64 * total) / 100.0;
            self.count = 0;
        }
        for i in 0..product.len() {
            let index = self.products.iter().position(|&x| x == product[i]).unwrap();
            total += self.prices[index] as f64 * amount[i] as f64;
        }
        return total;
    }
}

/**
 * Your Cashier object will be instantiated and called as such:
 * let obj = Cashier::new(n, discount, products, prices);
 * let ret_1: f64 = obj.get_bill(product, amount);
 */
fn main() {
    let mut obj = Cashier::new(
        3,
        50,
        vec![1, 2, 3, 4, 5, 6, 7],
        vec![100, 200, 300, 400, 300, 200, 100],
    );
    let ret_1: f64 = obj.get_bill(vec![1, 2], vec![1, 2]);
    assert_eq!(ret_1, 500.0);
    let ret_2: f64 = obj.get_bill(vec![3, 7], vec![10, 10]);
    assert_eq!(ret_2, 4000.0);
    let ret_3: f64 = obj.get_bill(vec![1, 2, 3, 4, 5, 6, 7], vec![1, 1, 1, 1, 1, 1, 1]);
    assert_eq!(ret_3, 800.0);
    let ret_4: f64 = obj.get_bill(vec![4], vec![10]);
    assert_eq!(ret_4, 4000.0);
    let ret_5: f64 = obj.get_bill(vec![7, 3], vec![10, 10]);
    assert_eq!(ret_5, 4000.0);
    let ret_6: f64 = obj.get_bill(vec![7, 5, 3, 1, 6, 4, 2], vec![10, 10, 10, 9, 9, 9, 7]);
    assert_eq!(ret_6, 7350.0);
    let ret_7: f64 = obj.get_bill(vec![2, 3, 5], vec![5, 3, 2]);
    assert_eq!(ret_7, 2500.0);
}
