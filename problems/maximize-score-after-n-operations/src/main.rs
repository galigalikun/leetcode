fn main() {
    assert_eq!(Solution::max_score(vec![1, 2]), 1);
    assert_eq!(Solution::max_score(vec![3,4,6,8]), 11);
    assert_eq!(Solution::max_score(vec![1,2,3,4,5,6]), 14);
}

struct Solution;
impl Solution {
    fn gcd(a: i32, b: i32) -> i32 {
        if a == 0 {
            return b;
        }
        if b == 0 {
            return a;
        }
        let mut a = a;
        let mut b = b;
        while b != 0 {
            let t = b;
            b = a % b;
            a = t;
        }
        a
    }
    pub fn max_score(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort_unstable();
        let mut sum = 0;
        let mut count = 0;
        for i in (0..nums.len()).rev() {
            sum += nums[i];
            if sum <= 0 {
                break;
            }
            count += 1;
        }
        if count == 0 {
            return 0;
        }
        let mut result = 0;
        for i in (0..count).rev() {
            result += nums[i];
        }
        result
    }
}
