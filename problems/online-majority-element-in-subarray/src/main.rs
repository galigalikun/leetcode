struct MajorityChecker {
    data: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MajorityChecker {
    fn new(arr: Vec<i32>) -> Self {
        MajorityChecker { data: arr }
    }

    fn query(&self, left: i32, right: i32, threshold: i32) -> i32 {
        let mut count = 0;
        let mut candidate = 0;
        for i in left..=right {
            if count == 0 {
                candidate = self.data[i as usize];
                count = 1;
            } else if self.data[i as usize] == candidate {
                count += 1;
            } else {
                count -= 1;
            }
        }
        count = 0;
        for i in left..=right {
            if self.data[i as usize] == candidate {
                count += 1;
            }
        }
        if count >= threshold {
            return candidate;
        }
        return -1;
    }
}

/**
 * Your MajorityChecker object will be instantiated and called as such:
 * let obj = MajorityChecker::new(arr);
 * let ret_1: i32 = obj.query(left, right, threshold);
 */
fn main() {
    let obj = MajorityChecker::new(vec![1, 1, 2, 2, 1, 1]);
    assert_eq!(obj.query(0, 5, 4), 1);
    assert_eq!(obj.query(0, 3, 3), -1);
    assert_eq!(obj.query(2, 3, 2), 2);
}
