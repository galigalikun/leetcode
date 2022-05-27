struct MyLinkedList {

}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyLinkedList {

    fn new() -> Self {
        MyLinkedList {

        }
    }

    fn get(&self, index: i32) -> i32 {
        return -1;
    }

    fn add_at_head(&self, val: i32) {

    }

    fn add_at_tail(&self, val: i32) {

    }

    fn add_at_index(&self, index: i32, val: i32) {

    }

    fn delete_at_index(&self, index: i32) {

    }
}

/**
 * Your MyLinkedList object will be instantiated and called as such:
 * let obj = MyLinkedList::new();
 * let ret_1: i32 = obj.get(index);
 * obj.add_at_head(val);
 * obj.add_at_tail(val);
 * obj.add_at_index(index, val);
 * obj.delete_at_index(index);
 */
fn main() {
    let obj = MyLinkedList::new();
    obj.add_at_head(1);
    obj.add_at_tail(3);
    obj.add_at_index(1, 2);
    assert_eq!(obj.get(1), 2);
    obj.delete_at_index(1);
    assert_eq!(obj.get(1), 3);
}
