fn main() {
    assert_eq!(Solution::largest_altitude(vec![-5, 1, 5, 0, -7]), 1);
    assert_eq!(Solution::largest_altitude(vec![-4, -3, -2, -1, 4, 3, 2]), 0);
}

struct Solution;
impl Solution {
    pub fn largest_altitude(gain: Vec<i32>) -> i32 {
        return gain
            .iter()
            .fold((0, 0), |(max, sum), x| {
                let sum = sum + x;
                (max.max(sum), sum)
            })
            .0;
    }
}
