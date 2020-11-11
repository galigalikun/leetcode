fn main() {
    assert_eq!(Solution::plus_one(vec![1, 2, 3]), vec![1, 2, 4]);
    assert_eq!(Solution::plus_one(vec![4, 3, 2, 1]), vec![4, 3, 2, 2]);
    assert_eq!(Solution::plus_one(vec![0]), vec![1]);
}

pub struct Solution {}
impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut i = 0;
        return digits
            .iter()
            .map(|&d| {
                i += 1;
                if i == digits.len() {
                    d + 1
                } else {
                    d
                }
            })
            .collect::<Vec<i32>>();
    }
}
