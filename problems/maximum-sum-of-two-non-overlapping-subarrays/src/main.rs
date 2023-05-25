fn main() {
    assert_eq!(
        Solution::max_sum_two_no_overlap(vec![0, 6, 5, 2, 2, 5, 1, 9, 4], 1, 2),
        20
    );
    assert_eq!(
        Solution::max_sum_two_no_overlap(vec![3, 8, 1, 3, 2, 1, 8, 9, 0], 3, 2),
        29
    );
    assert_eq!(
        Solution::max_sum_two_no_overlap(vec![2, 1, 5, 6, 0, 9, 5, 0, 3, 8], 4, 3),
        31
    );
}

struct Solution;
impl Solution {
    pub fn max_sum_two_no_overlap(nums: Vec<i32>, first_len: i32, second_len: i32) -> i32 {
        let mut max = 0;
        for i in 0..nums.len() {
            let mut first = 0;
            let mut second = 0;
            for j in 0..first_len {
                if i + j as usize >= nums.len() {
                    break;
                }
                first += nums[i + j as usize];
            }
            for j in 0..second_len {
                if i + j as usize >= nums.len() {
                    break;
                }
                second += nums[i + j as usize];
            }
            if first + second > max {
                max = first + second;
            }
        }
        return max;
    }
}
