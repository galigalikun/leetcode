struct MapSum {}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MapSum {
    fn new() -> Self {
        MapSum {}
    }

    fn insert(&self, key: String, val: i32) {}

    fn sum(&self, prefix: String) -> i32 {
        return 0;
    }
}

/**
 * Your MapSum object will be instantiated and called as such:
 * let obj = MapSum::new();
 * obj.insert(key, val);
 * let ret_2: i32 = obj.sum(prefix);
 */

fn main() {
    let obj = MapSum::new();
    obj.insert("apple".to_string(), 3);
    assert_eq!(obj.sum("ap".to_string()), 3);
    obj.insert("app".to_string(), 2);
    assert_eq!(obj.sum("ap".to_string()), 5);
}
