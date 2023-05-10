fn main() {
    assert_eq!(
        Solution::prefixes_div_by5(vec![0, 1, 1]),
        vec![true, false, false]
    );
    assert_eq!(
        Solution::prefixes_div_by5(vec![1, 1, 1]),
        vec![false, false, false]
    );
}

struct Solution;
impl Solution {
    pub fn prefixes_div_by5(nums: Vec<i32>) -> Vec<bool> {
        let mut result = vec![];
        let mut num = 0;
        for i in nums {
            num = (num * 2 + i) % 5;
            result.push(num == 0);
        }
        return result;
    }
}
