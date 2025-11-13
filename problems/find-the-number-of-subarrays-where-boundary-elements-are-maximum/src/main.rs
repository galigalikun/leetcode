fn main() {
    assert_eq!(Solution::number_of_subarrays(vec![1, 4, 3, 3, 2]), 6);
    assert_eq!(Solution::number_of_subarrays(vec![3, 3, 3]), 6);
    assert_eq!(Solution::number_of_subarrays(vec![1]), 1);
}

struct Solution;
impl Solution {
    pub fn number_of_subarrays(nums: Vec<i32>) -> i64 {
        let mut count = vec![0; 2];
        count[0] = 1;
        let mut result = 0i64;
        let mut prefix_sum = 0;

        for &num in &nums {
            prefix_sum ^= num & 1;
            result += count[prefix_sum as usize];
            count[prefix_sum as usize] += 1;
        }

        result
    }
}
