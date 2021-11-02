use std::collections::HashMap;
struct LRUCache {
    pub capacity: usize,
    pub priority: Vec<i32>,
    pub data: HashMap<i32, i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl LRUCache {
    fn new(capacity: i32) -> Self {
        LRUCache {
            capacity: capacity as usize,
            priority: Vec::with_capacity(capacity as usize),
            data: HashMap::with_capacity(capacity as usize),
        }
    }

    fn get(&mut self, key: i32) -> i32 {
        if let Some(&v) = self.data.get(&key) {
            if let Some(p) = self.priority.iter().position(|&x| x == key) {
                self.priority.remove(p);
                self.priority.push(key);
            }
            return v;
        }
        return -1;
    }

    fn put(&mut self, key: i32, value: i32) {
        if let Some(v) = self.data.get_mut(&key) {
            if let Some(p) = self.priority.iter().position(|&x| x == key) {
                self.priority.remove(p);
                self.priority.push(key);
            }
            *v = value;
        } else {
            if self.data.len() == self.capacity {
                let v = self.priority.remove(0);
                self.data.remove(&v);
            }
            self.priority.push(key);
            self.data.insert(key, value);
        }
    }
}

/**
 * Your LRUCache object will be instantiated and called as such:
 * let obj = LRUCache::new(capacity);
 * let ret_1: i32 = obj.get(key);
 * obj.put(key, value);
 */
fn main() {
    // let mut obj = LRUCache::new(2);
    // obj.put(1, 1); // cache is {1=1}
    // obj.put(2, 2); // cache is {1=1, 2=2}
    // assert_eq!(obj.get(1), 1);
    // obj.put(3, 3); // LRU key was 2, evicts key 2, cache is {1=1, 3=3}
    // assert_eq!(obj.get(2), -1);
    // obj.put(4, 4); // LRU key was 1, evicts key 1, cache is {4=4, 3=3}
    // assert_eq!(obj.get(1), -1);
    // assert_eq!(obj.get(3), 3);
    // assert_eq!(obj.get(4), 4);

    let mut obj = LRUCache::new(3);
    obj.put(1, 1);
    obj.put(2, 2);
    obj.put(3, 3);
    obj.put(4, 4);
    assert_eq!(obj.get(4), 4);
    assert_eq!(obj.get(3), 3);
    assert_eq!(obj.get(2), 2);
    assert_eq!(obj.get(1), -1);
    obj.put(5, 5);

    assert_eq!(obj.get(1), -1);
    assert_eq!(obj.get(2), 2);
    assert_eq!(obj.get(3), 3);
    assert_eq!(obj.get(4), -1);
    assert_eq!(obj.get(5), 5);
}
