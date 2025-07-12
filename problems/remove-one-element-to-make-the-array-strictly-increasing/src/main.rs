fn main() {
    assert_eq!(Solution::can_be_increasing(vec![1, 2, 10, 5, 7]), true);
    assert_eq!(Solution::can_be_increasing(vec![2, 3, 1, 2]), false);
    assert_eq!(Solution::can_be_increasing(vec![1, 1, 1]), false);
}

struct Solution;
impl Solution {
    fn can_be_increasing_with_limit(nums: Vec<i32>, limit: i32) -> bool {
        let mut count = 0;
        let mut prev = i32::MIN;

        for &num in nums.iter() {
            if num <= prev {
                count += 1;
                if count > limit {
                    return false;
                }
                if count == limit {
                    continue;
                }
            }
            prev = num;
        }

        true
    }
    pub fn can_be_increasing(nums: Vec<i32>) -> bool {
        return Self::can_be_increasing_with_limit(nums, 1);
    }
}
