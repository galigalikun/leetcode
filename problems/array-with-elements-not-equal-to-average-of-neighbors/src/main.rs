fn main() {
    assert_eq!(
        Solution::rearrange_array(vec![3, 1, -2, -5, 2, -4]),
        vec![3, -2, 1, -5, 2, -4]
    );
    assert_eq!(
        Solution::rearrange_array(vec![6, 2, 0, 9, 7]),
        vec![9, 7, 6, 2, 0]
    );
}

struct Solution;
impl Solution {
    pub fn rearrange_array(nums: Vec<i32>) -> Vec<i32> {
        let mut pos = vec![];
        let mut neg = vec![];
        for &num in &nums {
            if num > 0 {
                pos.push(num);
            } else {
                neg.push(num);
            }
        }
        let mut res = vec![];
        for i in 0..pos.len() {
            res.push(pos[i]);
            res.push(neg[i]);
        }
        res
    }
}
