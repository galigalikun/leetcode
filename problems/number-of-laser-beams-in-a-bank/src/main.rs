fn main() {
    assert_eq!(
        Solution::number_of_beams(vec![
            "011001".to_string(),
            "000000".to_string(),
            "010100".to_string(),
            "001000".to_string()
        ]),
        8
    );
    assert_eq!(
        Solution::number_of_beams(vec![
            "000".to_string(),
            "111".to_string(),
            "000".to_string(),
        ]),
        0
    );
}

struct Solution;
impl Solution {
    pub fn number_of_beams(bank: Vec<String>) -> i32 {
        return bank
            .iter()
            .filter(|row| row.contains('1'))
            .map(|row| row.chars().filter(|&c| c == '1').count() as i32)
            .fold((0, 0), |(total, prev), curr| (total + prev * curr, curr))
            .0;
    }
}
