fn main() {
    assert_eq!(Solution::count_valid_selections(vec![1, 0, 2, 0, 3]), 2);
    assert_eq!(
        Solution::count_valid_selections(vec![2, 3, 4, 0, 4, 1, 0]),
        0
    );
}

struct Solution;
impl Solution {
    pub fn count_valid_selections(nums: Vec<i32>) -> i32 {
        let mut count = 0;
        let n = nums.len();
        for mask in 0..(1 << n) {
            let mut selected = Vec::new();
            for i in 0..n {
                if (mask & (1 << i)) != 0 {
                    selected.push(nums[i]);
                }
            }
            if selected.is_empty() {
                continue;
            }
            let sum: i32 = selected.iter().sum();
            let prod: i32 = selected.iter().product();
            if sum > prod {
                count += 1;
            }
        }
        count
    }
}
