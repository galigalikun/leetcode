fn main() {
    assert_eq!(Solution::hamming_distance(1, 4), 2);
    assert_eq!(Solution::hamming_distance(3, 1), 1);
}

struct Solution {}
impl Solution {
    pub fn hamming_distance(x: i32, y: i32) -> i32 {
        return format!("{0:b}", x ^ y)
            .chars()
            .filter(|&x| x == '1')
            .count() as i32;
    }
}
