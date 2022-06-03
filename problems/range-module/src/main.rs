struct RangeModule {
    ranges: Vec<(i32, i32)>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RangeModule {

    fn new() -> Self {
        RangeModule {
            ranges: Vec::new(),
        }
    }

    fn add_range(&self, left: i32, right: i32) {
        let mut ranges = self.ranges.clone();
        let mut i = 0;
        while i < ranges.len() {
            let (l, r) = ranges[i];
            if l > right {
                break;
            }
            if left <= l && right >= r {
                ranges.remove(i);
            } else if left <= l && right < r {
                ranges[i].1 = right;
                break;
            } else if left > l && right >= r {
                ranges[i].0 = left;
                break;
            } else if left > l && right < r {
                ranges.insert(i, (left, right));
                break;
            }
            i += 1;
        }
    }

    fn query_range(&self, left: i32, right: i32) -> bool {
        let mut ranges = self.ranges.clone();
        let mut i = 0;
        while i < ranges.len() {
            let (l, r) = ranges[i];
            if l > right {
                break;
            }
            if left <= l && right >= r {
                return true;
            } else if left <= l && right < r {
                return false;
            } else if left > l && right >= r {
                ranges[i].1 = right;
                break;
            } else if left > l && right < r {
                ranges.insert(i, (left, right));
                break;
            }
            i += 1;
        }
        return false;
    }

    fn remove_range(&self, left: i32, right: i32) {
        let mut ranges = self.ranges.clone();
        let mut i = 0;
        while i < ranges.len() {
            let (l, r) = ranges[i];
            if l > right {
                break;
            }
            if left <= l && right >= r {
                ranges.remove(i);
            } else if left <= l && right < r {
                ranges[i].1 = left - 1;
            } else if left > l && right >= r {
                ranges[i].0 = right + 1;
            } else if left > l && right < r {
                ranges.insert(i, (left, right));
                break;
            }
            i += 1;
        }
    }
}

/**
 * Your RangeModule object will be instantiated and called as such:
 * let obj = RangeModule::new();
 * obj.add_range(left, right);
 * let ret_2: bool = obj.query_range(left, right);
 * obj.remove_range(left, right);
 */
fn main() {
    let obj = RangeModule::new();
    obj.add_range(10, 20);
    obj.remove_range(14, 16);
    assert_eq!(obj.query_range(10, 14), true);
    assert_eq!(obj.query_range(13, 15), false);
    assert_eq!(obj.query_range(16, 17), true);
}
