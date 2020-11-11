fn main() {
    assert_eq!(Solution::plus_one(vec![1, 2, 3]), vec![1, 2, 4]);
}

pub struct Solution {}
impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut i = 0;
        let aaa = digits
            .into_iter()
            .map(|d| {
                i += 1;
                if i == digits.len() {
                    d + 1
                } else {
                    d
                }
            });
            // .collect::<Vec<i32>>();
        return digits;
    }
}
