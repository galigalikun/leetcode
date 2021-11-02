fn main() {
    assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
    assert_eq!(Solution::two_sum(vec![3, 2, 4], 6), vec![1, 2]);
    assert_eq!(Solution::two_sum(vec![3, 3], 6), vec![0, 1]);
}

pub struct Solution {}

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let c = nums.len();
        for i in 0..c - 1 {
            for x in i + 1..c {
                let num = nums[i] + nums[x];
                if num == target {
                    return vec![i as i32, x as i32];
                }
            }
        }

        return vec![0];
    }
}
