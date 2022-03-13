fn main() {
    assert_eq!(
        Solution::find_max_consecutive_ones(vec![1, 1, 0, 1, 1, 1]),
        3
    );
    assert_eq!(
        Solution::find_max_consecutive_ones(vec![1, 0, 1, 1, 0, 1]),
        2
    );
}

struct Solution {}
impl Solution {
    pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
        let mut result = 0;
        let mut v = 0;
        for n in nums {
            if n == 1 {
                v += 1;
            } else {
                result = std::cmp::max(result, v);
                v = 0;
            }
        }
        result = std::cmp::max(result, v);
        return result;
    }
}
