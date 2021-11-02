fn main() {
    assert_eq!(Solution::circular_array_loop(vec![2, -1, 1, 2, 2]), true);
    assert_eq!(Solution::circular_array_loop(vec![-1, 2]), false);
    assert_eq!(
        Solution::circular_array_loop(vec![-2, 1, -1, -2, -2]),
        false
    );
    assert_eq!(Solution::circular_array_loop(vec![3, 1, 2]), true);
}

pub struct Solution {}
// https://medium.com/@mithlesh.kumar.4257/leetcode-457-circular-array-loop-fdcec9ea5746
impl Solution {
    fn next(nums: Vec<i32>, i: usize) -> usize {
        let n = nums.len();
        return ((n as i32 + nums[i] + i as i32) as usize) % n;
    }
    pub fn circular_array_loop(nums: Vec<i32>) -> bool {
        let mut nn = nums;
        for i in 0..nn.len() {
            nn[i] %= nn.len() as i32;
        }
        for i in 0..nn.len() {
            let mut slow = i;
            let mut fast = i;
            while nn[slow] * nn[Solution::next(nn.clone(), fast)] > 0
                && nn[slow] * nn[Solution::next(nn.clone(), Solution::next(nn.clone(), fast))] > 0
            {
                slow = Solution::next(nn.clone(), slow);
                fast = Solution::next(nn.clone(), Solution::next(nn.clone(), fast));
                if slow == fast {
                    if slow == Solution::next(nn.clone(), slow) {
                        return false;
                    }
                    return true;
                }
            }
            let mut j = i;
            let v = nn[i];
            while nn[j] * v > 0 {
                let nexx = Solution::next(nn.clone(), j);
                nn[j] = 0;
                j = nexx;
            }
        }
        return false;
    }
}
