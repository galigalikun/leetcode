struct CustomStack {
    stack: Vec<i32>,
    max_size: i32,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl CustomStack {

    fn new(maxSize: i32) -> Self {
        CustomStack{
            stack:vec![],
            max_size:maxSize
        }
    }
    
    fn push(&mut self, x: i32) {
        if self.stack.len() < self.max_size as usize {
            self.stack.push(x);
        }
    }
    
    fn pop(&mut self) -> i32 {
        if self.stack.len() > 0 {
            return self.stack.pop().unwrap();
        }
        -1
    }
    
    fn increment(&mut self, k: i32, val: i32) {
        let mut i = 0;
        while i < k as usize && i < self.stack.len() {
            self.stack[i] += val;
            i += 1;
        }
    }
}

/**
 * Your CustomStack object will be instantiated and called as such:
 * let obj = CustomStack::new(maxSize);
 * obj.push(x);
 * let ret_2: i32 = obj.pop();
 * obj.increment(k, val);
 */
fn main() {
    let mut obj = CustomStack::new(3);
    obj.push(1);
    obj.push(2);
    assert_eq!(2, obj.pop());
    obj.push(2);
    obj.push(3);
    obj.push(4);
    obj.increment(5, 100);
    obj.increment(2, 100);
    assert_eq!(103, obj.pop());
    assert_eq!(202, obj.pop());
    assert_eq!(201, obj.pop());
    assert_eq!(-1, obj.pop());
}
