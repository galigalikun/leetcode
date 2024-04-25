fn main() {
    assert_eq!(Solution::max_score("011101".to_string()), 5);
    assert_eq!(Solution::max_score("00111".to_string()), 5);
    assert_eq!(Solution::max_score("1111".to_string()), 3);
}

struct Solution;
impl Solution {
    pub fn max_score(s: String) -> i32 {
        let mut max = 0;
        let mut zeros = 0;
        let mut ones = 0;
        for i in 0..s.len() {
            if s.chars().nth(i).unwrap() == '1' {
                ones += 1;
            }
        }
        for i in 0..s.len() - 1 {
            if s.chars().nth(i).unwrap() == '0' {
                zeros += 1;
            } else {
                ones -= 1;
            }
            max = max.max(zeros + ones);
        }
        return max;
    }
}
