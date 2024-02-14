fn main() {
    assert_eq!(
        Solution::smaller_numbers_than_current(vec![8, 1, 2, 2, 3]),
        vec![4, 0, 1, 1, 3]
    );
    assert_eq!(
        Solution::smaller_numbers_than_current(vec![6, 5, 4, 8]),
        vec![2, 1, 0, 3]
    );
    assert_eq!(
        Solution::smaller_numbers_than_current(vec![7, 7, 7, 7]),
        vec![0, 0, 0, 0]
    );
}

struct Solution;
impl Solution {
    pub fn smaller_numbers_than_current(nums: Vec<i32>) -> Vec<i32> {
        let mut result = vec![];
        for i in 0..nums.len() {
            let mut count = 0;
            for j in 0..nums.len() {
                if nums[i] > nums[j] {
                    count += 1;
                }
            }
            result.push(count);
        }
        return result;
    }
}
