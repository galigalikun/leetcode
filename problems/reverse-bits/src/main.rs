fn main() {
    assert_eq!(
        Solution::reverse_bits(0b00000010100101000001111010011100u32),
        0b00111001011110000010100101000000u32
    );

    assert_eq!(
        Solution::reverse_bits(0b11111111111111111111111111111101u32),
        0b10111111111111111111111111111111u32
    );
}

pub struct Solution {}
impl Solution {
    pub fn reverse_bits(x: u32) -> u32 {
        let num = &format!("{:#034b}", x)[2..]
            .chars()
            .rev()
            .collect::<String>();
        return u32::from_str_radix(num, 2).unwrap();
    }
}
