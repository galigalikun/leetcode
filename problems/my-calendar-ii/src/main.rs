struct MyCalendarTwo {
    calendar: Vec<(i32, i32)>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyCalendarTwo {

    fn new() -> Self {
        MyCalendarTwo {
            calendar: Vec::new(),
        }
    }

    fn book(&mut self, start: i32, end: i32) -> bool {
        let mut i = 0;
        while i < self.calendar.len() {
            if self.calendar[i].0 >= end {
                break;
            }
            if self.calendar[i].1 >= start {
                return false;
            }
            i += 1;
        }
        self.calendar.push((start, end));
        return true;
    }
}

/**
 * Your MyCalendarTwo object will be instantiated and called as such:
 * let obj = MyCalendarTwo::new();
 * let ret_1: bool = obj.book(start, end);
 */
fn main() {
    let mut obj = MyCalendarTwo::new();
    assert_eq!(obj.book(10, 20), true);
    assert_eq!(obj.book(50, 60), true);
    assert_eq!(obj.book(10, 40), true);
    assert_eq!(obj.book(5, 15), false);
    assert_eq!(obj.book(5, 10), true);
    assert_eq!(obj.book(25, 55), true);
}
