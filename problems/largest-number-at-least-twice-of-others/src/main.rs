fn main() {
    assert_eq!(Solution::dominant_index(vec![3, 6, 1, 0]), 1);
    assert_eq!(Solution::dominant_index(vec![1, 2, 3, 4]), -1);
}

struct Solution {}
impl Solution {
    pub fn dominant_index(nums: Vec<i32>) -> i32 {
        let mut ans = -1;
        let m = nums.iter().max().unwrap();
        for (i, n) in nums.iter().enumerate() {
            if m == n {
                ans = i as i32;
            } else if *m < *n * 2 {
                return -1;
            }
        }
        return ans;
    }
}
