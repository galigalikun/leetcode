struct RecentCounter {
    requests:Vec<i32>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RecentCounter {

    fn new() -> Self {
        RecentCounter {
            requests:vec![],
         }
    }

    fn ping(&mut self, t: i32) -> i32 {
        self.requests.push(t);
        return 0;
    }
}

/**
 * Your RecentCounter object will be instantiated and called as such:
 * let obj = RecentCounter::new();
 * let ret_1: i32 = obj.ping(t);
 */
fn main() {
    let mut obj = RecentCounter::new();
    assert_eq!(1, obj.ping(1));
    assert_eq!(2, obj.ping(100));
    assert_eq!(3, obj.ping(3001));
    assert_eq!(3, obj.ping(3002));
}
