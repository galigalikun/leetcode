struct MyCircularQueue {
    data:Vec<Option<i32>>
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyCircularQueue {

    fn new(k: i32) -> Self {
        MyCircularQueue { data: vec![None;k as usize] }
    }

    fn en_queue(&mut self, value: i32) -> bool {
        if let Some(p) = self.data.iter().position(|x| x == &None) {
            self.data[p] = Some(value);
            return true;
        } else {
            return false;
        }
    }

    fn de_queue(&mut self) -> bool {
        if let Some(p) = self.data.iter().position(|x| x != &None) {
            self.data[p] = None;
            self.data.sort_by(|a, b| if a == &None && b != &None {
                std::cmp::Ordering::Greater
            } else if a != &None && b == &None {
                std::cmp::Ordering::Less
            } else {
                std::cmp::Ordering::Equal
            });
            return true;
        } else {
            return false;
        }
    }

    fn front(&self) -> i32 {
        return if let Some(v) = self.data.first().unwrap_or(&None) {
            *v
        } else {
            -1
        };
    }

    fn rear(&self) -> i32 {
        return if let Some(v) = self.data.iter().rfind(|&x| x != &None).unwrap_or(&None) {
            *v
        } else {
            -1
        };
    }

    fn is_empty(&self) -> bool {
        return if let Some(_p) = self.data.iter().position(|x| x != &None) {
            false
        } else {
            true
        };
    }

    fn is_full(&self) -> bool {
        return if let Some(_p) = self.data.iter().position(|x| x == &None) {
            false
        } else {
            true
        };
    }
}

/**
 * Your MyCircularQueue object will be instantiated and called as such:
 * let obj = MyCircularQueue::new(k);
 * let ret_1: bool = obj.en_queue(value);
 * let ret_2: bool = obj.de_queue();
 * let ret_3: i32 = obj.front();
 * let ret_4: i32 = obj.rear();
 * let ret_5: bool = obj.is_empty();
 * let ret_6: bool = obj.is_full();
 */
fn main() {
    let mut obj = MyCircularQueue::new(3);
    assert_eq!(obj.en_queue(1), true);
    assert_eq!(obj.en_queue(2), true);
    assert_eq!(obj.en_queue(3), true);
    assert_eq!(obj.en_queue(4), false);
    assert_eq!(obj.rear(), 3);
    assert_eq!(obj.is_full(), true);
    assert_eq!(obj.de_queue(), true);
    assert_eq!(obj.en_queue(4), true);
    assert_eq!(obj.rear(), 4);

    let mut obj = MyCircularQueue::new(8);
    assert_eq!(obj.en_queue(3), true);
    assert_eq!(obj.en_queue(9), true);
    assert_eq!(obj.en_queue(5), true);
    assert_eq!(obj.en_queue(0), true);
    assert_eq!(obj.de_queue(), true);
    assert_eq!(obj.de_queue(), true);
    assert_eq!(obj.is_empty(), false);
    assert_eq!(obj.is_empty(), false);
    assert_eq!(obj.rear(), 0);
    assert_eq!(obj.rear(), 0);
    assert_eq!(obj.de_queue(), true);

    let mut obj = MyCircularQueue::new(2);
    assert_eq!(obj.en_queue(4), true);
    assert_eq!(obj.rear(), 4);
    assert_eq!(obj.en_queue(9), true);
    assert_eq!(obj.de_queue(), true);
    assert_eq!(obj.front(), 9);
    assert_eq!(obj.de_queue(), true);
    assert_eq!(obj.de_queue(), false);
    assert_eq!(obj.is_empty(), true);
    assert_eq!(obj.de_queue(), false);
    assert_eq!(obj.en_queue(6), true);
    assert_eq!(obj.en_queue(4), true);
}
