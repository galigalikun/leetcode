fn main() {
    assert_eq!(Solution::array_pair_sum(vec![1, 4, 3, 2]), 4);
    assert_eq!(Solution::array_pair_sum(vec![6, 2, 6, 5, 1, 2]), 9);
}

struct Solution {}
impl Solution {
    pub fn array_pair_sum(nums: Vec<i32>) -> i32 {
        let mut num = nums;
        num.sort();
        let mut ans = 0;
        for i in (0..num.len()).step_by(2) {
            ans += std::cmp::min(num[i], num[i + 1]);
        }
        return ans;
    }
}
