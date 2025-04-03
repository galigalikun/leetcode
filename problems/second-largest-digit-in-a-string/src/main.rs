fn main() {
    assert_eq!(Solution::second_highest("dfa12321afd".to_string()), 2);
    assert_eq!(Solution::second_highest("abc1111".to_string()), -1);
    assert_eq!(Solution::second_highest("ck077".to_string()), 0);
}

struct Solution;
impl Solution {
    pub fn second_highest(s: String) -> i32 {
        let mut ab = s
            .chars()
            .filter(|c| c.is_ascii_digit())
            .map(|c| c.to_digit(10).unwrap() as i32)
            .collect::<std::collections::HashSet<_>>()
            .into_iter()
            .collect::<Vec<_>>();
        ab.sort();
        ab.reverse();

        if ab.len() < 2 {
            return -1;
        }
        let mut second_highest = -1;
        for i in (0..ab.len()).rev() {
            if ab[i] != ab[ab.len() - 1] {
                second_highest = ab[i];
                break;
            }
        }
        if second_highest == -1 {
            return -1;
        }
        second_highest
    }
}
