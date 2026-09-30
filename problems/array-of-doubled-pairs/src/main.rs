fn main() {
    assert_eq!(Solution::can_reorder_doubled(vec![3, 1, 3, 6]), false);
    assert_eq!(Solution::can_reorder_doubled(vec![2, 1, 2, 6]), false);
    assert_eq!(Solution::can_reorder_doubled(vec![4, -2, 2, -4]), true);
}

struct Solution;
impl Solution {
    pub fn can_reorder_doubled(arr: Vec<i32>) -> bool {
        use std::collections::HashMap;

        let mut values: Vec<i64> = arr.into_iter().map(i64::from).collect();
        values.sort_unstable_by_key(|x| x.abs());

        let mut counts = HashMap::new();
        for &x in &values {
            *counts.entry(x).or_insert(0usize) += 1;
        }

        for x in values {
            let count = counts.get_mut(&x).unwrap();
            if *count == 0 {
                continue;
            }
            *count -= 1;

            match counts.get_mut(&(2 * x)) {
                Some(count) if *count > 0 => *count -= 1,
                _ => return false,
            }
        }

        true
    }
}
