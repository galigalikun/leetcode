fn main() {
    assert_eq!(
        Solution::decode_ciphertext("ch   ie   pr".to_string(), 3),
        "cipher".to_string()
    );
    assert_eq!(
        Solution::decode_ciphertext("iveo    eed   l te   olc".to_string(), 4),
        "i love leetcode".to_string()
    );
    assert_eq!(
        Solution::decode_ciphertext("coding".to_string(), 1),
        "coding".to_string()
    );
}

struct Solution;
impl Solution {
    pub fn decode_ciphertext(encoded_text: String, rows: i32) -> String {
        let n = encoded_text.len() as i32;
        let cols = n / rows;
        let mut ans = String::new();
        for i in 0..cols {
            let mut j = 0;
            while i + j * (cols - 1) < n {
                let c = encoded_text.as_bytes()[(i + j * (cols - 1)) as usize] as char;
                if c != ' ' {
                    ans.push(c);
                }
                j += 1;
            }
        }
        ans
    }
}
