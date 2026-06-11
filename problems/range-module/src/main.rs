use std::collections::BTreeMap;

struct RangeModule {
    // start -> end for disjoint, sorted half-open intervals [start, end)
    ranges: BTreeMap<i32, i32>,
}

impl RangeModule {
    fn new() -> Self {
        Self {
            ranges: BTreeMap::new(),
        }
    }

    fn add_range(&mut self, left: i32, right: i32) {
        if left >= right {
            return;
        }

        let mut merged_left = left;
        let mut merged_right = right;

        if let Some((&start, &end)) = self.ranges.range(..=left).next_back() {
            if end >= left {
                merged_left = start.min(merged_left);
                merged_right = end.max(merged_right);
                self.ranges.remove(&start);
            }
        }

        let overlapping_starts: Vec<i32> = self
            .ranges
            .range(merged_left..=merged_right)
            .map(|(&start, _)| start)
            .collect();

        for start in overlapping_starts {
            if let Some(end) = self.ranges.remove(&start) {
                merged_right = merged_right.max(end);
            }
        }

        self.ranges.insert(merged_left, merged_right);
    }

    fn query_range(&self, left: i32, right: i32) -> bool {
        if left >= right {
            return true;
        }

        if let Some((_, &end)) = self.ranges.range(..=left).next_back() {
            return end >= right;
        }
        false
    }

    fn remove_range(&mut self, left: i32, right: i32) {
        if left >= right {
            return;
        }

        if let Some((&start, &end)) = self.ranges.range(..left).next_back() {
            if end > left {
                if end > right {
                    self.ranges.insert(start, left);
                    self.ranges.insert(right, end);
                    return;
                }
                self.ranges.insert(start, left);
            }
        }

        let overlapping_starts: Vec<i32> = self
            .ranges
            .range(left..right)
            .map(|(&start, _)| start)
            .collect();

        for start in overlapping_starts {
            let Some(end) = self.ranges.remove(&start) else {
                continue;
            };

            if end > right {
                self.ranges.insert(right, end);
                break;
            }
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
    let mut obj = RangeModule::new();
    obj.add_range(10, 20);
    obj.remove_range(14, 16);
    assert_eq!(obj.query_range(10, 14), true);
    assert_eq!(obj.query_range(13, 15), false);
    assert_eq!(obj.query_range(16, 17), true);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_case() {
        let mut rm = RangeModule::new();
        rm.add_range(10, 20);
        rm.remove_range(14, 16);
        assert!(rm.query_range(10, 14));
        assert!(!rm.query_range(13, 15));
        assert!(rm.query_range(16, 17));
    }

    #[test]
    fn merges_touching_intervals() {
        let mut rm = RangeModule::new();
        rm.add_range(1, 2);
        rm.add_range(2, 5);
        assert!(rm.query_range(1, 5));
    }

    #[test]
    fn remove_splits_interval() {
        let mut rm = RangeModule::new();
        rm.add_range(1, 10);
        rm.remove_range(3, 7);
        assert!(rm.query_range(1, 3));
        assert!(!rm.query_range(3, 7));
        assert!(rm.query_range(7, 10));
    }
}
