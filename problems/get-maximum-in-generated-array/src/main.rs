fn main() {
    assert_eq!(Solution::get_maximum_generated(7), 3);
    assert_eq!(Solution::get_maximum_generated(2), 1);
    assert_eq!(Solution::get_maximum_generated(3), 2);
}

struct Solution;
impl Solution {
    pub fn get_maximum_generated(n: i32) -> i32 {
        let mut nums = vec![0; (n + 1) as usize];
        if n == 0 {
            return 0;
        }
        nums[1] = 1;
        let mut max = 1;
        for i in 0..n as usize {
            if 2 * i <= n as usize {
                nums[2 * i] = nums[i];
                max = max.max(nums[2 * i]);
            }
            if 2 * i + 1 <= n as usize {
                nums[2 * i + 1] = nums[i] + nums[i + 1];
                max = max.max(nums[2 * i + 1]);
            }
        }
        max
    }
}
