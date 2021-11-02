fn main() {
    assert_eq!(Solution::decode_string("3[a]2[bc]".to_string()), "aaabcbc");
    assert_eq!(Solution::decode_string("3[a2[c]]".to_string()), "accaccacc");
    assert_eq!(
        Solution::decode_string("2[abc]3[cd]ef".to_string()),
        "abcabccdcdcdef"
    );
    assert_eq!(
        Solution::decode_string("abc3[cd]xyz".to_string()),
        "abccdcdcdxyz"
    );
}

pub struct Solution {}
// https://qiita.com/KueharX/items/4228d2a815922b1631be
impl Solution {
    pub fn decode_string(s: String) -> String {
        let mut result = "".to_string();
        let mut stack = vec![];
        let mut num = 0;
        for c in s.chars() {
            match c {
                '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                    num = 10 * num + c as i32 - 48;
                }
                '[' => {
                    stack.push(result);
                    stack.push(num.to_string());
                    result = "".to_string();
                    num = 0;
                }
                ']' => {
                    let n = stack.pop();
                    let pre = stack.pop();
                    result = format!(
                        "{}{}",
                        pre.unwrap(),
                        vec![result; n.unwrap().parse().unwrap()].join("")
                    );
                }
                _ => {
                    result = format!("{}{}", result, c);
                }
            };
        }
        return result;
    }
}
