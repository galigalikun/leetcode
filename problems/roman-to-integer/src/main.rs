fn main() {
    assert_eq!(Solution::roman_to_int("III".to_string()), 3);
    assert_eq!(Solution::roman_to_int("IV".to_string()), 4);
    assert_eq!(Solution::roman_to_int("IX".to_string()), 9);
    assert_eq!(Solution::roman_to_int("LVIII".to_string()), 58);
    assert_eq!(Solution::roman_to_int("MCMXCIV".to_string()), 1994);
    assert_eq!(Solution::roman_to_int("DCXXI".to_string()), 621);
}

pub struct Solution {}

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut num = 0;
        let mut i = 0;
        let mut x = 0;
        let mut c = 0;
        let mut prev = '\0';
        for ch in s.as_str().chars() {
            match ch {
                'I' => {
                    num += 1;
                    i += 1;
                }
                'V' => {
                    if prev == 'I' {
                        num += -1 - 1 * i;
                    }
                    num += 5;
                }
                'X' => {
                    if prev == 'I' {
                        num += -1 - 1 * i;
                    }
                    x += 10;
                    num += 10;
                }
                'L' => {
                    if prev == 'X' {
                        num += -10 - 1 * x;
                    }
                    num += 50;
                }
                'C' => {
                    if prev == 'X' {
                        num += -10 - 1 * x;
                    }
                    c += 100;
                    num += 100;
                }
                'D' => {
                    if prev == 'C' {
                        num += -100 - 1 * c;
                    }
                    num += 500;
                }
                'M' => {
                    if prev == 'C' {
                        num += -100 - 1 * c;
                    }
                    num += 1000;
                }
                _ => {
                    panic!("debug {}", ch);
                }
            }
            prev = ch;
        }
        return num;
    }
}
