fn main() {
    assert_eq!(Solution::decode_at_index("leet2code3".to_string(), 10), "o");
    assert_eq!(Solution::decode_at_index("ha22".to_string(), 5), "h");
    assert_eq!(Solution::decode_at_index("a2345678999999999999999".to_string(), 1), "a");
}

struct Solution;
impl Solution {
    pub fn decode_at_index(s: String, k: i32) -> String {
        let mut decoded_len: u64 = 0;
        for ch in s.chars() {
            if ch.is_ascii_digit() {
                decoded_len *= ch.to_digit(10).unwrap() as u64;
            } else {
                decoded_len += 1;
            }
        }

        let mut kth: u64 = k as u64;
        for ch in s.chars().rev() {
            kth %= decoded_len;
            if kth == 0 && ch.is_ascii_alphabetic() {
                return ch.to_string();
            }

            if ch.is_ascii_digit() {
                decoded_len /= ch.to_digit(10).unwrap() as u64;
            } else {
                decoded_len -= 1;
            }
        }

        String::new()
    }
}
