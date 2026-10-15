use std::collections::HashMap;

fn main() {
    assert_eq!(Solution::num_squareful_perms(vec![1, 17, 8]), 2);
    assert_eq!(Solution::num_squareful_perms(vec![2, 2, 2]), 1);
    assert_eq!(Solution::num_squareful_perms(vec![1, 2, 3]), 0);
}

struct Solution;
impl Solution {
    fn is_squareful(a: i32, b: i32) -> bool {
        let x = (a as i64) + (b as i64);
        let root = (x as f64).sqrt();
        root == root.floor()
    }

    pub fn num_squareful_perms(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }

        let mut freq = HashMap::new();
        for &value in &nums {
            *freq.entry(value).or_insert(0i32) += 1;
        }

        let mut values: Vec<i32> = freq.keys().copied().collect();
        values.sort_unstable();

        let mut memo: HashMap<(Option<usize>, Vec<i32>), i64> = HashMap::new();
        let mut counts = vec![0; values.len()];
        for value in &nums {
            let index = values.binary_search(value).unwrap();
            counts[index] += 1;
        }

        fn dfs(
            last: Option<usize>,
            counts: Vec<i32>,
            values: &[i32],
            memo: &mut HashMap<(Option<usize>, Vec<i32>), i64>,
        ) -> i64 {
            if let Some(&cached) = memo.get(&(last, counts.clone())) {
                return cached;
            }

            let remaining = counts.iter().sum::<i32>();
            if remaining == 0 {
                memo.insert((last, counts.clone()), 1);
                return 1;
            }

            let mut total = 0i64;
            for i in 0..values.len() {
                if counts[i] == 0 {
                    continue;
                }

                if let Some(prev) = last {
                    if !Solution::is_squareful(values[prev], values[i]) {
                        continue;
                    }
                }

                let mut next = counts.clone();
                next[i] -= 1;
                total += dfs(Some(i), next, values, memo);
            }

            memo.insert((last, counts.clone()), total);
            total
        }

        let total = dfs(None, counts, &values, &mut memo);
        total as i32
    }
}
