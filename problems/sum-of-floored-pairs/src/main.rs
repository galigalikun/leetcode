fn main() {
    assert_eq!(Solution::sum_of_floored_pairs(vec![2, 5, 9]), 10);
    assert_eq!(
        Solution::sum_of_floored_pairs(vec![7, 7, 7, 7, 7, 7, 7]),
        49
    );
}

struct Solution;
impl Solution {
    pub fn sum_of_floored_pairs(nums: Vec<i32>) -> i32 {
        return nums
            .iter()
            .enumerate()
            .map(|(i, &x)| {
                nums.iter()
                    .filter(|&&y| y >= x)
                    .map(|&y| i32::min(x, y))
                    .sum::<i32>()
            })
            .sum();
    }
}
