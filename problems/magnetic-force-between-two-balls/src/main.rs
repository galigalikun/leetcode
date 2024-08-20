fn main() {
    assert_eq!(Solution::max_distance(vec![1, 2, 3, 4, 7], 3), 3);
    assert_eq!(
        Solution::max_distance(vec![5, 4, 3, 2, 1, 1000000000], 2),
        999999999
    );
}

struct Solution;
impl Solution {
    fn check(position: &Vec<i32>, mid: i32, m: i32) -> bool {
        let mut count = 1;
        let mut last = position[0];
        for i in 1..position.len() {
            if position[i] - last >= mid {
                count += 1;
                last = position[i];
            }
        }
        count >= m
    }
    pub fn max_distance(position: Vec<i32>, m: i32) -> i32 {
        let mut position = position;
        position.sort();
        let n = position.len();
        let mut left = 1;
        let mut right = position[n - 1] - position[0];
        while left < right {
            let mid = (left + right + 1) / 2;
            if Self::check(&position, mid, m) {
                left = mid;
            } else {
                right = mid - 1;
            }
        }
        left
    }
}
