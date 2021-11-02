use std::collections::HashMap;
struct LFUCache {
    capacity: usize,
    map: HashMap<i32, i32>,
    cache: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl LFUCache {
    fn new(capacity: i32) -> Self {
        LFUCache {
            capacity: capacity as usize,
            map: HashMap::new(),
            cache: vec![],
        }
    }

    fn get(&mut self, key: i32) -> i32 {
        let n = self.cache.len();
        if let Some(p) = self.cache.iter().position(|&x| x == key) {
            self.cache.swap(p, n - 1);
        }
        return *self.map.get(&key).unwrap_or_else(|| &-1);
    }

    fn put(&mut self, key: i32, value: i32) {
        if self.cache.len() >= self.capacity {
            let p = self.cache.remove(0);
            self.map.remove(&p);
        }
        self.map.insert(key, value);
        self.cache.push(key);
    }
}

/**
 * Your LFUCache object will be instantiated and called as such:
 * let obj = LFUCache::new(capacity);
 * let ret_1: i32 = obj.get(key);
 * obj.put(key, value);
 */
fn main() {
    let mut obj = LFUCache::new(2);
    obj.put(1, 1);
    obj.put(2, 2);
    assert_eq!(obj.get(1), 1);
    obj.put(3, 3);
    assert_eq!(obj.get(2), -1);
    assert_eq!(obj.get(3), 3);
    obj.put(4, 4);
    assert_eq!(obj.get(1), -1);
    assert_eq!(obj.get(3), 3);
    assert_eq!(obj.get(4), 4);
}
