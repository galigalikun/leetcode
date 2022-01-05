fn main() {
    assert_eq!(Solution::array_nesting(vec![5, 4, 0, 3, 1, 6, 2]), 4);
    assert_eq!(Solution::array_nesting(vec![0, 1, 2]), 1);
    assert_eq!(Solution::array_nesting(vec![0, 2, 1]), 2);
}

// https://studyalgorithms.com/array/array-nesting/
struct Solution {}
impl Solution {
    pub fn array_nesting(nums: Vec<i32>) -> i32 {
        let mut visited = vec![false; nums.len()];
        let mut ans = 0;
        for i in 0..nums.len() {
            if !visited[i] {
                let mut start = nums[i];
                let mut count = 0;
                loop {
                    start = nums[start as usize];
                    count += 1;
                    visited[start as usize] = true;
                    if start == nums[i] {
                        break;
                    }
                }
                ans = std::cmp::max(ans, count);
            }
        }
        return ans;
    }
}
