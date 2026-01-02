fn main() {
    assert_eq!(Solution::find_x_sum(vec![1,1,2,2,3,4,2,3], 6, 2), vec![6,10,12]);
    assert_eq!(Solution::find_x_sum(vec![3,8,7,8,7,5], 2, 2), vec![11,15,15,15,12]);
}

struct Solution;
impl Solution {
    pub fn find_x_sum(nums: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
        let n = nums.len();
        let mut nums = nums;
        for _i in 0..n {
            let mut new_nums = vec![0; n];
            for j in 0..n {
                let left = if j >= 1 { nums[j - 1] } else { 0 };
                let right = if j + 1 < n { nums[j + 1] } else { 0 };
                new_nums[j] = left + right + k;
            }
            if x as usize == _i + 1 {
                return new_nums;
            }
            nums = new_nums;
        }
        return vec![];
    }
}
