fn main() {
    assert_eq!(Solution::find_length_of_lcis(vec![1, 3, 5, 4, 7]), 3);
    assert_eq!(Solution::find_length_of_lcis(vec![2, 2, 2, 2, 2]), 1);
    assert_eq!(
        Solution::find_length_of_lcis(vec![1, 3, 5, 4, 2, 3, 4, 5]),
        4
    );
}

struct Solution {}
impl Solution {
    pub fn find_length_of_lcis(nums: Vec<i32>) -> i32 {
        let mut prev = nums[0];
        let mut count = 1;
        let mut ans = 1;
        for i in 1..nums.len() {
            if prev < nums[i] {
                count += 1;
            } else {
                ans = std::cmp::max(ans, count);
                count = 1;
            }
            prev = nums[i];
        }
        ans = std::cmp::max(ans, count);
        return ans;
    }
}
