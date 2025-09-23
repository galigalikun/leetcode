fn main() {
    assert_eq!(Solution::find_middle_index(vec![2, 3, -1, 8, 4]), 3);
    assert_eq!(Solution::find_middle_index(vec![1, -1, 4]), 2);
    assert_eq!(Solution::find_middle_index(vec![2, 5]), -1);
}

struct Solution;
impl Solution {
    pub fn find_middle_index(nums: Vec<i32>) -> i32 {
        let total: i32 = nums.iter().sum();
        let mut left_sum = 0;
        for (i, &num) in nums.iter().enumerate() {
            if left_sum * 2 + num == total {
                return i as i32;
            }
            left_sum += num;
        }
        return -1;
    }
}
