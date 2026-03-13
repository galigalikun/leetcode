fn main() {
    assert_eq!(Solution::longest_balanced(vec![2, 5, 4, 3]), 4);
    assert_eq!(Solution::longest_balanced(vec![3, 2, 2, 5, 4]), 5);
    assert_eq!(Solution::longest_balanced(vec![1, 2, 3, 2]), 3);
}

struct Solution;
impl Solution {
    pub fn longest_balanced(nums: Vec<i32>) -> i32 {
        let mut count = 0;
        let mut max_length = 0;
        for num in nums {
            if num == 0 {
                count += 1;
            } else {
                count -= 1;
            }
            if count == 0 {
                max_length += 2;
            }
        }
        max_length
    }
}
