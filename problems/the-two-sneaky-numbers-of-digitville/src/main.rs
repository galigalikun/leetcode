fn main() {
    assert_eq!(Solution::get_sneaky_numbers(vec![0, 1, 1, 0]), vec![0, 1]);
    assert_eq!(
        Solution::get_sneaky_numbers(vec![0, 3, 2, 1, 3, 2]),
        vec![2, 3]
    );
    assert_eq!(
        Solution::get_sneaky_numbers(vec![7, 1, 5, 4, 3, 4, 6, 0, 9, 5, 8, 2]),
        vec![4, 5]
    );
}

struct Solution;
impl Solution {
    pub fn get_sneaky_numbers(nums: Vec<i32>) -> Vec<i32> {
        let mut result = Vec::new();
        for &num in &nums {
            if num % 2 == 0 && num % 3 == 0 {
                result.push(num);
            }
        }
        result
    }
}
