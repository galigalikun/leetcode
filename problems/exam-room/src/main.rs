struct ExamRoom {

}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl ExamRoom {

    fn new(n: i32) -> Self {
        ExamRoom {  }
    }

    fn seat(&self) -> i32 {
        return 0;
    }

    fn leave(&self, p: i32) {

    }
}

/**
 * Your ExamRoom object will be instantiated and called as such:
 * let obj = ExamRoom::new(n);
 * let ret_1: i32 = obj.seat();
 * obj.leave(p);
 */
fn main() {
    let obj = ExamRoom::new(10);
    assert_eq!(0, obj.seat());
    assert_eq!(9, obj.seat());
    assert_eq!(4, obj.seat());
    assert_eq!(2, obj.seat());
    obj.leave(4);
    assert_eq!(5, obj.seat());
}
