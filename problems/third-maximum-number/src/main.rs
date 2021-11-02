fn main() {
    assert_eq!(Solution::third_max(vec![3, 2, 1]), 1);
    assert_eq!(Solution::third_max(vec![1, 2]), 2);
    assert_eq!(Solution::third_max(vec![2, 2, 3, 1]), 1);
}

pub struct Solution {}
impl Solution {
    pub fn third_max(nums: Vec<i32>) -> i32 {
        let mut nn = nums;
        nn.sort_by(|a, b| b.cmp(a));
        let mut prev = None;
        let mut result = vec![];
        for n in nn {
            if prev != Some(n) {
                result.push(n);
            }
            prev = Some(n);
        }
        if result.len() > 2 {
            return result[2];
        }
        return result[0];
    }
}
