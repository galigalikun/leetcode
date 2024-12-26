fn main() {
    assert_eq!(Solution::number_of_matches(7), 6);
    assert_eq!(Solution::number_of_matches(14), 13);
}

struct Solution;
impl Solution {
    pub fn number_of_matches(n: i32) -> i32 {
        let mut n = n;
        let mut matches = 0;
        while n > 1 {
            matches += n / 2;
            n = (n + 1) / 2;
        }
        matches
    }
}
