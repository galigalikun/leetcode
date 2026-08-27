use std::collections::HashSet;

fn main() {
    assert_eq!(Solution::fair_candy_swap(vec![1, 1], vec![2, 2]), vec![1, 2]);
    assert_eq!(Solution::fair_candy_swap(vec![1, 2], vec![2, 3]), vec![1, 2]);
    assert_eq!(Solution::fair_candy_swap(vec![2], vec![1, 3]), vec![2, 3]);
}

struct Solution;
impl Solution {
    pub fn fair_candy_swap(alice_sizes: Vec<i32>, bob_sizes: Vec<i32>) -> Vec<i32> {
        let alice_sum: i32 = alice_sizes.iter().sum();
        let bob_sum: i32 = bob_sizes.iter().sum();
        let delta = (bob_sum - alice_sum) / 2;
        let bob_set: HashSet<i32> = bob_sizes.iter().copied().collect();

        alice_sizes
            .iter()
            .copied()
            .find(|&a| bob_set.contains(&(a + delta)))
            .map(|a| vec![a, a + delta])
            .unwrap_or_default()
    }
}
