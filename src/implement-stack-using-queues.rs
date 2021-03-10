struct MyStack {
    data: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyStack {
    /** Initialize your data structure here. */
    fn new() -> Self {
        MyStack { data: vec![] }
    }

    /** Push element x onto stack. */
    fn push(&mut self, x: i32) {
        self.data.push(x);
    }

    /** Removes the element on top of the stack and returns that element. */
    fn pop(&mut self) -> i32 {
        return self.data.pop().unwrap();
    }

    /** Get the top element. */
    fn top(&self) -> i32 {
        return *self.data.last().unwrap();
    }

    /** Returns whether the stack is empty. */
    fn empty(&self) -> bool {
        return self.data.is_empty();
    }
}

/**
 * Your MyStack object will be instantiated and called as such:
 * let obj = MyStack::new();
 * obj.push(x);
 * let ret_2: i32 = obj.pop();
 * let ret_3: i32 = obj.top();
 * let ret_4: bool = obj.empty();
 */
fn main() {
    let mut obj = MyStack::new();
    obj.push(1);
    obj.push(2);
    assert_eq!(obj.top(), 2); // return 2
    assert_eq!(obj.pop(), 2); // return 2
    assert_eq!(obj.empty(), false); // return False
}
