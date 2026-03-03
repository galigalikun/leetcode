fn main() {
    assert_eq!(Solution::minimum_pair_removal(vec![5, 2, 3, 1]), 2);
    assert_eq!(Solution::minimum_pair_removal(vec![1, 2, 2]), 0);
}

struct Solution;
impl Solution {
    pub fn minimum_pair_removal(nums: Vec<i32>) -> i32 {
        let mut count = std::collections::HashMap::new();
        for num in nums {
            *count.entry(num).or_insert(0) += 1;
        }
        let mut ans = 0;
        for (_, c) in count {
            ans += c / 2;
        }
        ans
    }
}
