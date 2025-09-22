fn main() {
    assert_eq!(
        Solution::number_of_unique_good_subsequences("001".to_string()),
        2
    );
    assert_eq!(
        Solution::number_of_unique_good_subsequences("11".to_string()),
        2
    );
    assert_eq!(
        Solution::number_of_unique_good_subsequences("101".to_string()),
        5
    );
}

struct Solution;
impl Solution {
    pub fn number_of_unique_good_subsequences(binary: String) -> i32 {
        return if binary.contains('0') { 1 } else { 0 }
            + (1..=binary.len()).fold(0, |acc, _| (acc * 2 + 1) % 1_000_000_007);
    }
}
