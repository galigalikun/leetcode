use std::collections::HashMap;
struct MyCalendar {
    calendar: HashMap<i32, i32>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyCalendar {

    fn new() -> Self {
        MyCalendar {
            calendar: HashMap::new(),
        }
    }

    fn book(&mut self, start: i32, end: i32) -> bool {
        if self.calendar.contains_key(&start) && self.calendar.contains_key(&end) {
            return false;
        }
        if self.calendar.contains_key(&start) {
            if self.calendar[&start] >= end {
                return false;
            }
        }
        if self.calendar.contains_key(&end) {
            if self.calendar[&end] <= start {
                return false;
            }
        }
        self.calendar.insert(start, end);
        return true;
    }
}

/**
 * Your MyCalendar object will be instantiated and called as such:
 * let obj = MyCalendar::new();
 * let ret_1: bool = obj.book(start, end);
 */
fn main() {
    let mut obj = MyCalendar::new();
    assert_eq!(obj.book(10, 20), true);
    assert_eq!(obj.book(15, 25), false);
    assert_eq!(obj.book(20, 30), true);
}
