fn main() {
    assert_eq!(
        Solution::alphabet_board_path("leet".to_string()),
        "DDR!UURRR!!DDD!"
    );
    assert_eq!(
        Solution::alphabet_board_path("code".to_string()),
        "RR!DDRR!UUL!R!"
    );
    assert_eq!(
        Solution::alphabet_board_path("zdz".to_string()),
        "DDDDD!UUUUURRR!DDDDLLLD!"
    );
}

struct Solution;
impl Solution {
    fn get_position(c: char) -> (i32, i32) {
        let index = (c as u8 - 'a' as u8) as i32;
        return (index / 5, index % 5);
    }
    pub fn alphabet_board_path(target: String) -> String {
        let mut result = String::new();
        let mut current = (0, 0);
        for c in target.chars() {
            let next = Self::get_position(c);
            let (x, y) = (next.0 - current.0, next.1 - current.1);
            if x > 0 {
                result.push_str(&"D".repeat(x as usize));
            } else if x < 0 {
                result.push_str(&"U".repeat(x.abs() as usize));
            }
            if y > 0 {
                result.push_str(&"R".repeat(y as usize));
            } else if y < 0 {
                result.push_str(&"L".repeat(y.abs() as usize));
            }
            result.push_str("!");
            current = next;
        }
        return result;
    }
}
