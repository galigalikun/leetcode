use std::collections::HashMap;

struct TimeMap {
    map: HashMap<String, HashMap<i32, String>>
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl TimeMap {

    fn new() -> Self {
        TimeMap { map: HashMap::new() }
    }
    
    fn set(&mut self, key: String, value: String, timestamp: i32) {
        if let Some(m) = self.map.get_mut(&key) {
            m.insert(timestamp, value);
        } else {
            let mut m = HashMap::new();
            m.insert(timestamp, value);
            self.map.insert(key, m);
        }
    }
    
    fn get(&self, key: String, timestamp: i32) -> String {
        if let Some(m) = self.map.get(&key) {
            let mut k = i32::MIN;
            for p in m {
                if p.0 <= &timestamp {
                    k = std::cmp::max(k, *p.0);
                }
            }
            return m.get(&k).unwrap_or(&"".to_string()).to_string();
        }
        return "".to_string();
    }
}

/**
 * Your TimeMap object will be instantiated and called as such:
 * let obj = TimeMap::new();
 * obj.set(key, value, timestamp);
 * let ret_2: String = obj.get(key, timestamp);
 */
fn main() {
    let mut obj = TimeMap::new();
    obj.set("foo".to_string(), "bar".to_string(), 1);
    assert_eq!("bar", obj.get("foo".to_string(), 1));
    assert_eq!("bar", obj.get("foo".to_string(), 3));
    obj.set("foo".to_string(), "bar2".to_string(), 4);
    assert_eq!("bar2", obj.get("foo".to_string(), 4));
    assert_eq!("bar2", obj.get("foo".to_string(), 5));
}
