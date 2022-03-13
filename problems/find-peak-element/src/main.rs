fn main() {
    assert_eq!(Solution::find_peak_element(vec![1, 2, 3, 1]), 2);
}

struct Solution {}
impl Solution {
    pub fn find_peak_element(nums: Vec<i32>) -> i32 {
        let mut max = std::i32::MIN;
        let mut idx: i32 = 0;
        for i in 0..nums.len() {
            if nums[i] > max {
                max = nums[i];
                idx = i as i32;
            }
        }

        return idx;
    }
}
