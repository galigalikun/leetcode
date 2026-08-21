fn main() {
    assert_eq!(Solution::min_eating_speed(vec![3,6,7,11], 8), 4);
    assert_eq!(Solution::min_eating_speed(vec![30,11,23,4,20], 5), 30);
    assert_eq!(Solution::min_eating_speed(vec![30,11,23,4,20], 6), 23);
}

struct Solution;
impl Solution {
    pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
        let mut left = 1;
        let mut right = piles.iter().copied().max().unwrap_or(1);

        while left < right {
            let mid = left + (right - left) / 2;
            let hours = piles
                .iter()
                .map(|&pile| (pile + mid - 1) / mid)
                .map(i64::from)
                .sum::<i64>();

            if hours <= i64::from(h) {
                right = mid;
            } else {
                left = mid + 1;
            }
        }

        left
    }
}
