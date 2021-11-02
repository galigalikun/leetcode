struct MyQueue {
    data: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyQueue {
    /** Initialize your data structure here. */
    fn new() -> Self {
        MyQueue { data: vec![] }
    }

    /** Push element x to the back of queue. */
    fn push(&mut self, x: i32) {
        self.data.push(x);
    }

    /** Removes the element from in front of queue and returns that element. */
    fn pop(&mut self) -> i32 {
        return self.data.remove(0);
    }

    /** Get the front element. */
    fn peek(&self) -> i32 {
        if let Some(&t) = self.data.first() {
            return t;
        }
        return 0;
    }

    /** Returns whether the queue is empty. */
    fn empty(&self) -> bool {
        return self.data.is_empty();
    }
}

/**
 * Your MyQueue object will be instantiated and called as such:
 * let obj = MyQueue::new();
 * obj.push(x);
 * let ret_2: i32 = obj.pop();
 * let ret_3: i32 = obj.peek();
 * let ret_4: bool = obj.empty();
 */
fn main() {
    let mut obj = MyQueue::new();
    obj.push(1);
    obj.push(2);
    assert_eq!(obj.peek(), 1);
    assert_eq!(obj.pop(), 1);
    assert_eq!(obj.empty(), false);
}
