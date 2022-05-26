struct MyHashMap {
    buckets: Vec<(i32, i32)>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyHashMap {
    fn new() -> Self {
        MyHashMap { buckets: vec![] }
    }

    fn put(&mut self, key: i32, value: i32) {
        for (i, (k, _v)) in self.buckets.iter().enumerate() {
            if k == &key {
                self.buckets[i] = (key, value);
                return;
            }
        }
        self.buckets.push((key, value));
    }

    fn get(&self, key: i32) -> i32 {
        for (k, v) in self.buckets.iter() {
            if *k == key {
                return *v;
            }
        }
        return -1;
    }

    fn remove(&mut self, key: i32) {
        for (i, (k, _v)) in self.buckets.iter().enumerate() {
            if *k == key {
                self.buckets.remove(i as usize);
                return;
            }
        }
    }
}

/**
 * Your MyHashMap object will be instantiated and called as such:
 * let obj = MyHashMap::new();
 * obj.put(key, value);
 * let ret_2: i32 = obj.get(key);
 * obj.remove(key);
 */
fn main() {
    let mut obj = MyHashMap::new();
    obj.put(1, 1);
    obj.put(2, 2);
    assert_eq!(obj.get(1), 1);
    assert_eq!(obj.get(3), -1);
    obj.put(2, 1);
    assert_eq!(obj.get(2), 1);
    obj.remove(2);
    assert_eq!(obj.get(2), -1);
}
