struct FreqStack {
    stack: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl FreqStack {
    fn new() -> Self {
        FreqStack { stack: vec![] }
    }

    fn push(&mut self, val: i32) {
        self.stack.push(val);
    }

    fn pop(&mut self) -> i32 {
        return self.stack.pop().unwrap_or(0);
    }
}

/**
 * Your FreqStack object will be instantiated and called as such:
 * let obj = FreqStack::new();
 * obj.push(val);
 * let ret_2: i32 = obj.pop();
 */
fn main() {
    let mut obj = FreqStack::new();
    obj.push(5);
    obj.push(7);
    obj.push(5);
    obj.push(7);
    obj.push(4);
    obj.push(5);
    assert_eq!(5, obj.pop());
    assert_eq!(7, obj.pop());
    assert_eq!(5, obj.pop());
    assert_eq!(4, obj.pop());
}
