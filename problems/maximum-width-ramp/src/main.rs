fn main() {
    assert_eq!(Solution::max_width_ramp(vec![6, 0, 8, 2, 1, 5]), 4);
    assert_eq!(
        Solution::max_width_ramp(vec![9, 8, 1, 0, 1, 9, 4, 0, 4, 1]),
        7
    );
}

struct Solution;
impl Solution {
    pub fn max_width_ramp(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n < 2 {
            return 0;
        }

        let mut dec_stack: Vec<usize> = Vec::new();
        for i in 0..n {
            if dec_stack.is_empty() || nums[i] < nums[*dec_stack.last().unwrap()] {
                dec_stack.push(i);
            }
        }

        let mut max_width = 0usize;
        for j in (0..n).rev() {
            while let Some(&i) = dec_stack.last() {
                if nums[i] <= nums[j] {
                    max_width = max_width.max(j - i);
                    dec_stack.pop();
                } else {
                    break;
                }
            }
            if dec_stack.is_empty() {
                break;
            }
        }

        max_width as i32
    }
}
