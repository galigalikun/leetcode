struct MyCalendar {
    calendar: Vec<(i32, i32)>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyCalendar {

    fn new() -> Self {
        MyCalendar {
            calendar: Vec::new(),
        }
    }

    fn book(&mut self, start: i32, end: i32) -> bool {
        for &(existing_start, existing_end) in &self.calendar {
            // Overlap exists for half-open intervals when
            // start < existing_end && existing_start < end.
            if start < existing_end && existing_start < end {
                return false;
            }
        }

        self.calendar.push((start, end));
        true
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

#[cfg(test)]
mod tests {
    use super::MyCalendar;

    #[test]
    fn books_non_overlapping_events() {
        let mut calendar = MyCalendar::new();
        assert!(calendar.book(10, 20));
        assert!(calendar.book(20, 30));
        assert!(calendar.book(5, 10));
    }

    #[test]
    fn rejects_overlapping_events() {
        let mut calendar = MyCalendar::new();
        assert!(calendar.book(10, 20));
        assert!(!calendar.book(15, 25));
        assert!(!calendar.book(5, 15));
        assert!(!calendar.book(12, 18));
    }
}
