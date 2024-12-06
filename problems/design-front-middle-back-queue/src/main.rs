struct FrontMiddleBackQueue {
    data: Vec<i32>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl FrontMiddleBackQueue {

    fn new() -> Self {
        FrontMiddleBackQueue{
            data: Vec::new(),
        }
    }

    fn push_front(&mut self, val: i32) {
        self.data.insert(0, val);
    }

    fn push_middle(&mut self, val: i32) {
        let len = self.data.len();
        let mid = len / 2;
        self.data.insert(mid, val);
    }

    fn push_back(&mut self, val: i32) {
        self.data.push(val);
    }

    fn pop_front(&self) -> i32 {
        self.data.get(0).unwrap_or(&-1).clone()
    }

    fn pop_middle(&self) -> i32 {
        self.data.get(self.data.len() / 2).unwrap_or(&-1).clone()
    }

    fn pop_back(&self) -> i32 {
        self.data.last().unwrap_or(&-1).clone()
    }
}

/**
 * Your FrontMiddleBackQueue object will be instantiated and called as such:
 * let obj = FrontMiddleBackQueue::new();
 * obj.push_front(val);
 * obj.push_middle(val);
 * obj.push_back(val);
 * let ret_4: i32 = obj.pop_front();
 * let ret_5: i32 = obj.pop_middle();
 * let ret_6: i32 = obj.pop_back();
 */
fn main() {
    let mut obj = FrontMiddleBackQueue::new();
    obj.push_front(1);
    obj.push_back(2);
    obj.push_middle(3);
    obj.push_middle(4);
    assert_eq!(1, obj.pop_front());
    assert_eq!(3, obj.pop_middle());
    assert_eq!(4, obj.pop_middle());
    assert_eq!(2, obj.pop_back());
    assert_eq!(-1, obj.pop_front());
}
