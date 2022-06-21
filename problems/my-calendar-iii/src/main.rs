use std::collections::HashMap;

struct MyCalendarThree {
    calendar: HashMap<i32, i32>,
    max: i32,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyCalendarThree {

    fn new() -> Self {
        MyCalendarThree {
            calendar: HashMap::new(),
            max: 0,
        }
    }

    fn book(&mut self, start: i32, end: i32) -> i32 {
        let mut max = self.max;
        for k in start..end {
            let v = self.calendar.entry(k).or_insert(0);
            *v += 1;
            max = std::cmp::max(max, *v);
        }
        self.max = max;
        return max;
    }
}

/**
 * Your MyCalendarThree object will be instantiated and called as such:
 * let obj = MyCalendarThree::new();
 * let ret_1: i32 = obj.book(start, end);
 */
fn main() {
    let mut obj = MyCalendarThree::new();
    assert_eq!(obj.book(10, 20), 1);
    assert_eq!(obj.book(50, 60), 1);
    assert_eq!(obj.book(10, 40), 2);
    assert_eq!(obj.book(5, 15), 3);
    assert_eq!(obj.book(5, 10), 3);
    assert_eq!(obj.book(25, 55), 3);
    assert_eq!(obj.book(0, 1000000000), 4);
}
