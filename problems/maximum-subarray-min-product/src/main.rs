fn main() {
    assert_eq!(Solution::max_sum_min_product(vec![1, 2, 3, 2]), 14);
    assert_eq!(Solution::max_sum_min_product(vec![2, 3, 3, 1, 2]), 18);
    assert_eq!(Solution::max_sum_min_product(vec![3, 1, 5, 6, 4, 2]), 60);
}

struct Solution;
impl Solution {
    pub fn max_sum_min_product(nums: Vec<i32>) -> i32 {
        return nums
            .iter()
            .enumerate()
            .map(|(i, &x)| {
                let mut sum = 0;
                let mut min = x;
                for j in i..nums.len() {
                    sum += nums[j];
                    min = min.max(nums[j]);
                    if j > i {
                        break;
                    }
                }
                sum * min
            })
            .max()
            .unwrap_or(0);
    }
}
