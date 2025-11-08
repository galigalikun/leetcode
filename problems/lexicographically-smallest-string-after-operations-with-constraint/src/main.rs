fn main() {
    assert_eq!(Solution::get_smallest_string("zbbz".to_string(), 3), "aaaz");
    assert_eq!(Solution::get_smallest_string("xaxcd".to_string(), 4), "aawcd");
    assert_eq!(Solution::get_smallest_string("lol".to_string(), 0), "lol");
}

struct Solution;
impl Solution {
    pub fn get_smallest_string(s: String, k: i32) -> String {
        let mut s_chars: Vec<char> = s.chars().collect();
        let n = s_chars.len();
        let mut k = k;
        for i in 0..n {
            if k == 0 {
                break;
            }
            let current_char = s_chars[i];
            let distance_to_a = (current_char as u8 - b'a') as i32;
            if distance_to_a <= k {
                s_chars[i] = 'a';
                k -= distance_to_a;
            } else {
                let new_char = ((current_char as u8) - (k as u8)) as char;
                s_chars[i] = new_char;
                k = 0;
            }
        }
        if k == 0 {
            return s_chars.into_iter().collect();
        }
        s_chars.sort();
        return s_chars.into_iter().collect();
    }
}
