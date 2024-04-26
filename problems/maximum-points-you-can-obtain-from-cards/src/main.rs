fn main() {
    assert_eq!(Solution::max_score(vec![1, 2, 3, 4, 5, 6, 1], 3), 12);
    assert_eq!(Solution::max_score(vec![2, 2, 2], 2), 4);
    assert_eq!(Solution::max_score(vec![9, 7, 7, 9, 7, 7, 9], 7), 55);
}

struct Solution;
impl Solution {
    pub fn max_score(card_points: Vec<i32>, k: i32) -> i32 {
        let n = card_points.len();
        let k = k as usize;
        let mut sum = 0;
        for i in 0..k {
            sum += card_points[i];
        }
        let mut max = sum;
        for i in 0..k {
            sum += card_points[n - 1 - i] - card_points[k - 1 - i];
            max = max.max(sum);
        }
        return max;
    }
}
