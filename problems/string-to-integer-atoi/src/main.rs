fn main() {
    assert_eq!(Solution::my_atoi("42".to_string()), 42);
    assert_eq!(Solution::my_atoi("   -42".to_string()), -42);
    assert_eq!(Solution::my_atoi("4193 with words".to_string()), 4193);
    assert_eq!(Solution::my_atoi("words and 987".to_string()), 0);
    assert_eq!(Solution::my_atoi("-91283472332".to_string()), -2147483648);
    assert_eq!(Solution::my_atoi("3.14159".to_string()), 3);
    assert_eq!(Solution::my_atoi("+-12".to_string()), 0);
    assert_eq!(Solution::my_atoi("".to_string()), 0);
    assert_eq!(Solution::my_atoi("+".to_string()), 0);
    assert_eq!(Solution::my_atoi("21474836460".to_string()), 2147483647);
    assert_eq!(Solution::my_atoi("1a".to_string()), 1);
}

pub struct Solution {}
impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let ss = s.trim();
        if ss.len() == 0 {
            return 0;
        }
        let mut signed = false;
        let mut plus = true;
        let mut idx = 0;
        let mut str = Vec::new();
        for c in ss.chars() {
            if idx == 0 {
                match c {
                    '.' => {
                        return 0;
                    }
                    '+' => {
                        signed = true;
                    }
                    '-' => {
                        signed = true;
                        plus = false;
                    }
                    '0' => {}
                    '1' => {}
                    '2' => {}
                    '3' => {}
                    '4' => {}
                    '5' => {}
                    '6' => {}
                    '7' => {}
                    '8' => {}
                    '9' => {}
                    _ => return 0,
                }
            } else if idx == 1 {
                match c {
                    '.' => {
                        if signed {
                            return 0;
                        }
                        break;
                    }
                    '0' => {}
                    '1' => {}
                    '2' => {}
                    '3' => {}
                    '4' => {}
                    '5' => {}
                    '6' => {}
                    '7' => {}
                    '8' => {}
                    '9' => {}
                    _ => break,
                }
            } else {
                match c {
                    '.' => break,
                    '0' => {}
                    '1' => {}
                    '2' => {}
                    '3' => {}
                    '4' => {}
                    '5' => {}
                    '6' => {}
                    '7' => {}
                    '8' => {}
                    '9' => {}
                    _ => break,
                }
            }
            str.push(c);
            idx += 1;
        }

        if str.len() == 1 && signed {
            return 0;
        }

        let n = match str.iter().collect::<String>().parse::<i32>() {
            Ok(x) => x as i32,
            Err(_) => {
                if plus {
                    i32::max_value()
                } else {
                    i32::min_value()
                }
            }
        };
        return n;
    }
}
