use std::collections::HashMap;

struct MapSum {
    map: HashMap<String, i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MapSum {
    fn new() -> Self {
        MapSum {
            map: HashMap::new(),
        }
    }

    fn insert(&mut self, key: String, val: i32) {
        self.map.insert(key, val);
    }

    fn sum(&self, prefix: String) -> i32 {
        let mut sum = 0;
        for (key, val) in self.map.iter() {
            if key.starts_with(&prefix) {
                sum +=*val;
            }
        }
        return sum;
    }
}

/**
 * Your MapSum object will be instantiated and called as such:
 * let obj = MapSum::new();
 * obj.insert(key, val);
 * let ret_2: i32 = obj.sum(prefix);
 */

fn main() {
    let mut obj = MapSum::new();
    obj.insert("apple".to_string(), 3);
    assert_eq!(obj.sum("ap".to_string()), 3);
    obj.insert("app".to_string(), 2);
    assert_eq!(obj.sum("ap".to_string()), 5);
}
