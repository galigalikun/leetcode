struct MyCircularDeque {
    data:Vec<i32>,
    size: usize,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyCircularDeque {

    fn new(k: i32) -> Self {
        MyCircularDeque { data: vec![], size: k as usize }
    }

    fn insert_front(&mut self, value: i32) -> bool {
        if self.is_full() {
            return false;
        }
        self.data.insert(0, value);
        return true;
    }

    fn insert_last(&mut self, value: i32) -> bool {
        if self.is_full() {
            return false;
        }
        self.data.push(value);
        return true;
    }

    fn delete_front(&mut self) -> bool {
        if self.data.len() > 0 {
            self.data.remove(0);
            return true;
        }
        return false;
    }

    fn delete_last(&mut self) -> bool {
        if self.data.pop() != None {
            return true;
        }
        return false;
    }

    fn get_front(&self) -> i32 {
        return *self.data.first().unwrap_or(&-1);
    }

    fn get_rear(&self) -> i32 {
        return *self.data.last().unwrap_or(&-1);
    }

    fn is_empty(&self) -> bool {
        return self.data.is_empty();
    }

    fn is_full(&self) -> bool {
        return if self.data.len() == self.size {
            true
        } else {
            false
        };
    }
}

/**
 * Your MyCircularDeque object will be instantiated and called as such:
 * let obj = MyCircularDeque::new(k);
 * let ret_1: bool = obj.insert_front(value);
 * let ret_2: bool = obj.insert_last(value);
 * let ret_3: bool = obj.delete_front();
 * let ret_4: bool = obj.delete_last();
 * let ret_5: i32 = obj.get_front();
 * let ret_6: i32 = obj.get_rear();
 * let ret_7: bool = obj.is_empty();
 * let ret_8: bool = obj.is_full();
 */
fn main() {
    let mut obj = MyCircularDeque::new(3);
    assert_eq!(obj.insert_last(1), true);
    assert_eq!(obj.insert_last(2), true);
    assert_eq!(obj.insert_front(3), true);
    assert_eq!(obj.insert_front(4), false);
    assert_eq!(obj.get_rear(), 2);
    assert_eq!(obj.is_full(), true);
    assert_eq!(obj.delete_last(), true);
    assert_eq!(obj.insert_front(4), true);
    assert_eq!(obj.get_front(), 4);

    let mut obj = MyCircularDeque::new(71);
    assert_eq!(obj.insert_last(47), true);
    assert_eq!(obj.delete_last(), true);
    assert_eq!(obj.delete_last(), false);
}
