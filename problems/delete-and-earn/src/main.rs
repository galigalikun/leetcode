fn main() {
    assert_eq!(Solution::delete_and_earn(vec![3, 4, 2]), 6);
    assert_eq!(Solution::delete_and_earn(vec![2, 2, 3, 3, 3, 4]), 9);
}

struct Solution {}
impl Solution {
    pub fn delete_and_earn(nums: Vec<i32>) -> i32 {
        let mut ans = 0;
        for (i, n) in nums.iter().enumerate() {
            ans = std::cmp::max(
                ans,
                nums[0..i]
                    .iter()
                    .filter(|&x| *x == n - 1)
                    .fold(0, |sum, a| sum + a)
                    + nums[i]
                    + nums[i + 1..]
                        .iter()
                        .filter(|&x| *x == n + 1)
                        .fold(0, |sum, a| sum + a),
            );
        }
        return ans;
    }
}
