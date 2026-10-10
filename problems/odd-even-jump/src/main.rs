use std::collections::BTreeMap;
use std::ops::Bound;

fn main() {
    assert_eq!(Solution::odd_even_jumps(vec![10, 13, 12, 14, 15]), 2);
    assert_eq!(Solution::odd_even_jumps(vec![2, 3, 1, 1, 4]), 3);
    assert_eq!(Solution::odd_even_jumps(vec![5, 1, 3, 4, 2]), 3);
    assert_eq!(Solution::odd_even_jumps(vec![2, 2, 2]), 3);
}

struct Solution;
impl Solution {
    pub fn odd_even_jumps(arr: Vec<i32>) -> i32 {
        let n = arr.len();
        if n == 0 {
            return 0;
        }

        let mut odd_next = vec![-1; n];
        let mut even_next = vec![-1; n];
        let mut values: BTreeMap<i32, usize> = BTreeMap::new();

        for i in (0..n).rev() {
            if i != n - 1 {
                let lower_bound = values.range(arr[i]..).next();
                if let Some((_, &idx)) = lower_bound {
                    odd_next[i] = idx as i32;
                }

                let upper_bound = values.range((Bound::Unbounded, Bound::Included(arr[i]))).next_back();
                if let Some((_, &idx)) = upper_bound {
                    even_next[i] = idx as i32;
                }
            }

            let value = arr[i];
            values.entry(value).and_modify(|idx| {
                if i < *idx {
                    *idx = i;
                }
            }).or_insert(i);
        }

        let mut good_odd = vec![false; n];
        let mut good_even = vec![false; n];
        good_odd[n - 1] = true;
        good_even[n - 1] = true;

        for i in (0..n - 1).rev() {
            if odd_next[i] != -1 && good_even[odd_next[i] as usize] {
                good_odd[i] = true;
            }
            if even_next[i] != -1 && good_odd[even_next[i] as usize] {
                good_even[i] = true;
            }
        }

        good_odd.iter().filter(|&&ok| ok).count() as i32
    }
}
