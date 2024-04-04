fn main() {
    assert_eq!(Solution::num_steps("1101".to_string()), 6);
    assert_eq!(Solution::num_steps("10".to_string()), 1);
    assert_eq!(Solution::num_steps("1".to_string()), 0);
}

struct Solution;
impl Solution {
    pub fn num_steps(s: String) -> i32 {
        let mut s = s;
        let mut count = 0;
        while s != "1".to_string() {
            if s.ends_with("0") {
                s.pop();
            } else {
                let mut carry = 1;
                let mut i = s.len() as i32 - 1;
                while carry == 1 && i >= 0 {
                    if s.chars().nth(i as usize).unwrap() == '1' {
                        s = format!(
                            "{}0{}",
                            s.chars().take(i as usize).collect::<String>(),
                            s.chars().skip(i as usize + 1).collect::<String>()
                        );
                    } else {
                        s = format!(
                            "{}1{}",
                            s.chars().take(i as usize).collect::<String>(),
                            s.chars().skip(i as usize + 1).collect::<String>()
                        );
                        carry = 0;
                    }
                    i -= 1;
                }
                if carry == 1 {
                    s = "1".to_string() + &s;
                }
            }
            count += 1;
        }
        return count;
    }
}
