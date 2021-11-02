fn main() {
    assert_eq!(
        Solution::top_k_frequent(vec![1, 1, 1, 2, 2, 3], 2),
        vec![1, 2]
    );

    assert_eq!(Solution::top_k_frequent(vec![1], 1), vec![1]);
}

pub struct Solution {}
use std::collections::HashMap;
impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut map = HashMap::new();
        for n in nums {
            if let Some(m) = map.get_mut(&n) {
                *m += 1;
            } else {
                map.insert(n, 1);
            }
        }

        let mut result = map.into_iter().map(|(n, c)| (n, c)).collect::<Vec<_>>();
        result.sort_by(|(_, a), (_, b)| b.cmp(a));

        return (&result[0..k as usize])
            .into_iter()
            .map(|(n, _)| *n)
            .collect::<Vec<_>>();
    }
}
