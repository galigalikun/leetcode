fn main() {
    assert_eq!(Solution::maximum_gap(vec![3, 6, 9, 1]), 3);
    assert_eq!(Solution::maximum_gap(vec![10]), 0);
    assert_eq!(Solution::maximum_gap(vec![100, 3, 2, 1]), 97);
}

struct Solution {}
impl Solution {
    pub fn maximum_gap(nums: Vec<i32>) -> i32 {
        let mut list = nums;
        list.sort();
        let mut result = 0;
        for n in 1..list.len() {
            let diff = list[n] - list[n - 1];
            if diff > result {
                result = diff;
            }
        }

        return result;
    }
}
