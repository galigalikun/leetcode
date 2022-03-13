fn main() {
    assert_eq!(Solution::count_bits(2), vec![0, 1, 1]);
    assert_eq!(Solution::count_bits(5), vec![0, 1, 1, 2, 1, 2]);
}

struct Solution {}
impl Solution {
    pub fn count_bits(n: i32) -> Vec<i32> {
        let mut result = vec![];
        for i in 0..=n {
            result.push(
                format!("{:0b}", i as usize)
                    .chars()
                    .fold(0, |sum, a| sum + a as i32 - 48),
            );
        }
        return result;
    }
}
