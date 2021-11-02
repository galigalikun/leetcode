struct MinStack {
    pub data: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {
    /** initialize your data structure here. */
    fn new() -> Self {
        MinStack { data: Vec::new() }
    }

    fn push(&mut self, x: i32) {
        self.data.push(x);
    }

    fn pop(&mut self) {
        self.data.pop();
    }

    fn top(&self) -> i32 {
        if let Some(&t) = self.data.last() {
            return t;
        }
        return 0;
    }

    fn get_min(&self) -> i32 {
        if let Some(&m) = self.data.iter().min() {
            return m;
        }
        return 0;
    }
}

/**
 * Your MinStack object will be instantiated and called as such:
 * let obj = MinStack::new();
 * obj.push(x);
 * obj.pop();
 * let ret_3: i32 = obj.top();
 * let ret_4: i32 = obj.get_min();
 */
fn main() {
    let mut obj = MinStack::new();
    obj.push(-2);
    obj.push(0);
    obj.push(-3);
    assert_eq!(obj.get_min(), -3);
    obj.pop();
    assert_eq!(obj.top(), 0);
    assert_eq!(obj.get_min(), -2);
}
