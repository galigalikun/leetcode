struct AllOne {
    data: std::collections::HashMap<String, i32>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl AllOne {

    fn new() -> Self {
        AllOne {
            data: std::collections::HashMap::new(),
        }
    }

    fn inc(&mut self, key: String) {
        let counter = self.data.entry(key).or_insert(0);
        *counter += 1;
    }

    fn dec(&mut self, key: String) {
        if let Some(counter) = self.data.get_mut(&key) {
            *counter -= 1;
            if *counter == 0 {
                self.data.remove(&key);
            }
        }
    }

    fn get_max_key(&self) -> String {
        self.data.iter().max_by_key(|entry| entry.1).map(|(key, _)| key.clone()).unwrap_or_default()
    }

    fn get_min_key(&self) -> String {
        self.data.iter().min_by_key(|entry| entry.1).map(|(key, _)| key.clone()).unwrap_or_default()
    }
}

/**
 * Your AllOne object will be instantiated and called as such:
 * let obj = AllOne::new();
 * obj.inc(key);
 * obj.dec(key);
 * let ret_3: String = obj.get_max_key();
 * let ret_4: String = obj.get_min_key();
 */
fn main() {
    let mut obj = AllOne::new();
    obj.inc("hello".to_string());
    obj.inc("hello".to_string());
    assert_eq!(obj.get_max_key(), "hello".to_string());
    assert_eq!(obj.get_min_key(), "hello".to_string());
    obj.inc("leet".to_string());
    assert_eq!(obj.get_max_key(), "hello".to_string());
    assert_eq!(obj.get_min_key(), "leet".to_string());
}
