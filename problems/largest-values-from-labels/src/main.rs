fn main() {
    assert_eq!(
        Solution::largest_vals_from_labels(vec![5, 4, 3, 2, 1], vec![1, 1, 2, 2, 3], 3, 1),
        9
    );
    assert_eq!(
        Solution::largest_vals_from_labels(vec![5, 4, 3, 2, 1], vec![1, 3, 3, 3, 2], 3, 2),
        12
    );
    assert_eq!(
        Solution::largest_vals_from_labels(vec![9, 8, 8, 7, 6], vec![0, 0, 0, 1, 1], 3, 1),
        16
    );
}

struct Solution;
impl Solution {
    pub fn largest_vals_from_labels(
        values: Vec<i32>,
        labels: Vec<i32>,
        num_wanted: i32,
        use_limit: i32,
    ) -> i32 {
        use std::collections::HashMap;
        let mut map = HashMap::new();
        let mut v = values.iter().zip(labels.iter()).collect::<Vec<_>>();
        v.sort_by(|a, b| b.0.cmp(a.0));
        let mut sum = 0;
        let mut count = 0;
        for (value, label) in v {
            if count == num_wanted {
                break;
            }
            if let Some(&c) = map.get(label) {
                if c == use_limit {
                    continue;
                }
            }
            sum += value;
            count += 1;
            *map.entry(label).or_insert(0) += 1;
        }
        return sum;
    }
}
