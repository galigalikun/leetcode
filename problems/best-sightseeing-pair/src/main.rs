fn main() {
    assert_eq!(
        Solution::max_score_sightseeing_pair(vec![8, 1, 5, 2, 6]),
        11
    );
    assert_eq!(Solution::max_score_sightseeing_pair(vec![1, 2]), 2);
}

struct Solution;
impl Solution {
    pub fn max_score_sightseeing_pair(values: Vec<i32>) -> i32 {
        let mut max = 0;
        let mut max_i = 0;
        for i in 0..values.len() {
            let v = values[i];
            let score = max_i + v - i as i32;
            if score > max {
                max = score;
            }
            if v + i as i32 > max_i {
                max_i = v + i as i32;
            }
        }
        return max;
    }
}
