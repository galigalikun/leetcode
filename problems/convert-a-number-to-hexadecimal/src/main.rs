fn main() {
    assert_eq!(Solution::to_hex(26), "1a");
    assert_eq!(Solution::to_hex(-1), "ffffffff");
    assert_eq!(Solution::to_hex(16), "10");
    assert_eq!(Solution::to_hex(-2), "fffffffe");
}

struct Solution {}
impl Solution {
    pub fn to_hex(num: i32) -> String {
        let mut nn = if num >= 0 {
            num as u32
        } else {
            std::u32::MAX - (-1 * num - 1) as u32
        };
        let mut result = vec![];
        while nn >= 16 {
            result.push(std::char::from_digit(nn % 16, 16).unwrap());
            nn = nn >> 4;
        }
        result.push(std::char::from_digit(nn % 16, 16).unwrap());
        return result.iter().rev().collect::<String>();
    }
}
