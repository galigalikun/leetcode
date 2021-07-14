fn main() {
    assert_eq!(
        Solution::max_envelopes(vec![vec![5, 4], vec![6, 4], vec![6, 7], vec![2, 3]]),
        3
    );

    assert_eq!(
        Solution::max_envelopes(vec![vec![1,1],vec![1,1],vec![1,1]]),
        1
    );

    assert_eq!(
        Solution::max_envelopes(vec![vec![4,5],vec![4,6],vec![6,7],vec![2,3],vec![1,1]]),
        4
    );
}

pub struct Solution {}
// https://dev.to/seanpgallivan/solution-russian-doll-envelopes-459i
use std::collections::HashMap;
impl Solution {
    pub fn max_envelopes(envelopes: Vec<Vec<i32>>) -> i32 {
        let mut e = envelopes;
        e.sort_by(|a, b| match a[0].cmp(&b[0]) {
            std::cmp::Ordering::Equal => b[1].cmp(&a[1]),
            other => other
        });

        let mut dp = HashMap::new();
        for i in 0..e.len() {
            let height = e[i][1];
            let mut left = 0;
            let mut right = dp.len();
            while left < right {
                let mid = (left + right) >> 1;
                if dp.get(&mid) < Some(&height) {
                    left = mid + 1;
                } else {
                    right = mid;
                }
            }

            dp.insert(left, height);
        }

        return dp.len() as i32;
    }
}
