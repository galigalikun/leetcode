fn main() {
    assert_eq!(
        Solution::hammingWeight(0b00000000000000000000000000001011u32),
        3
    );

    assert_eq!(
        Solution::hammingWeight(0b00000000000000000000000010000000u32),
        1
    );

    assert_eq!(
        Solution::hammingWeight(0b11111111111111111111111111111101u32),
        31
    );
}

pub struct Solution {}
impl Solution {
    pub fn hammingWeight(n: u32) -> i32 {
        return format!("{:#034b}", n)[2..]
            .chars()
            .filter(|&x| x == '1')
            .count() as i32;
    }
}
