fn main() {
    assert_eq!(Solution::get_maximum_consecutive(vec![1, 3]), 2);
    assert_eq!(Solution::get_maximum_consecutive(vec![1, 1, 1, 4]), 8);
    assert_eq!(Solution::get_maximum_consecutive(vec![1, 4, 10, 3, 1]), 20);
}

struct Solution;
impl Solution {
    pub fn get_maximum_consecutive(coins: Vec<i32>) -> i32 {
        return coins.into_iter().fold(1, |acc, coin| {
            if coin > acc {
                return acc;
            }
            acc + coin
        });
    }
}
