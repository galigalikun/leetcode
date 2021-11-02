fn main() {
    assert_eq!(Solution::convert_to_title(1), "A".to_string());
    assert_eq!(Solution::convert_to_title(28), "AB".to_string());
    assert_eq!(Solution::convert_to_title(701), "ZY".to_string());
}

pub struct Solution {}
impl Solution {
    pub fn convert_to_title(n: i32) -> String {
        let mut num = n;
        let mut col = "".to_string();
        loop {
            col = format!("{}{}", (((num - 1) % 26) + 65) as u8 as char, col);
            num = (num - 1) / 26;
            if num <= 0 {
                break;
            }
        }
        return col.to_string();
    }
}
