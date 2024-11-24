struct OrderedStream {
    data: Vec<String>,
    ptr: i32,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl OrderedStream {
    fn new(n: i32) -> Self {
        OrderedStream {
            data: vec!["".to_string(); n as usize],
            ptr: 0,
        }
    }

    fn insert(&mut self, id_key: i32, value: String) -> Vec<String> {
        let id_key = id_key - 1;
        self.data[id_key as usize] = value;
        let mut res = vec![];
        while self.ptr < self.data.len() as i32 && !self.data[self.ptr as usize].is_empty() {
            res.push(self.data[self.ptr as usize].clone());
            self.ptr += 1;
        }
        res
    }
}

/**
 * Your OrderedStream object will be instantiated and called as such:
 * let obj = OrderedStream::new(n);
 * let ret_1: Vec<String> = obj.insert(idKey, value);
 */
fn main() {
    let mut obj = OrderedStream::new(5);
    assert_eq!(obj.insert(3, "ccccc".to_string()), Vec::<String>::new());
    assert_eq!(obj.insert(1, "aaaaa".to_string()), vec!["aaaaa"]);
    assert_eq!(obj.insert(2, "bbbbb".to_string()), vec!["bbbbb", "ccccc"]);
    assert_eq!(obj.insert(5, "eeeee".to_string()), Vec::<String>::new());
    assert_eq!(obj.insert(4, "ddddd".to_string()), vec!["ddddd", "eeeee"]);
}
