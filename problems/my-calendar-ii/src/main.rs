struct MyCalendarTwo {
    booked: Vec<(i32, i32)>,
    overlaps: Vec<(i32, i32)>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyCalendarTwo {

    fn new() -> Self {
        MyCalendarTwo {
            booked: Vec::new(),
            overlaps: Vec::new(),
        }
    }

    fn book(&mut self, start: i32, end: i32) -> bool {
        for &(overlap_start, overlap_end) in &self.overlaps {
            if start < overlap_end && end > overlap_start {
                return false;
            }
        }

        for &(booked_start, booked_end) in &self.booked {
            if start < booked_end && end > booked_start {
                self.overlaps
                    .push((start.max(booked_start), end.min(booked_end)));
            }
        }

        self.booked.push((start, end));
        true
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
