fn main() {
    assert_eq!(
        Solution::build_array(vec![0, 2, 1, 5, 3, 4]),
        vec![0, 1, 2, 4, 5, 3]
    );
    assert_eq!(
        Solution::build_array(vec![5, 0, 1, 2, 3, 4]),
        vec![4, 5, 0, 1, 2, 3]
    );
}

struct Solution;
impl Solution {
    pub fn build_array(nums: Vec<i32>) -> Vec<i32> {
        return nums
            .iter()
            .enumerate()
            .map(|(_i, &num)| nums[num as usize])
            .collect();
    }
}
