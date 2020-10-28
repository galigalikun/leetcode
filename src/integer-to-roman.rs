fn main () {
    assert_eq!(Solution::int_to_roman(3), "III".to_string());
    assert_eq!(Solution::int_to_roman(4), "IV".to_string());
    assert_eq!(Solution::int_to_roman(9), "IX".to_string());
    assert_eq!(Solution::int_to_roman(20), "XX".to_string());
    assert_eq!(Solution::int_to_roman(58), "LVIII".to_string());
    assert_eq!(Solution::int_to_roman(1994), "MCMXCIV".to_string());
}

pub struct Solution {}
impl Solution {
    pub fn int_to_roman(num: i32) -> String {
        let mut i = 0;
        let mut n = num / (10 as i32).pow(i);
        let mut a = Vec::new();
        while n > 0 {
            let key = n % 10;
            if i == 0 {
                if 4 > key && key >= 1 {
                    let mut s = "I".to_string();
                    for _ in 1..key {
                        s.push_str("I");
                    }
                    a.push(s);
                } else if key == 4 {
                    a.push("IV".to_string());
                } else if key == 5 {
                    a.push("V".to_string());
                } else if 9 > key && key >= 6 {
                    let mut s = "V".to_string();
                    for _ in 6..=key {
                        s.push_str("I");
                    }
                    a.push(s);
                } else if key == 9 {
                    a.push("IX".to_string());
                }
            } else if i == 1 {
                if 4 > key && key >= 1 {
                    let mut s = "X".to_string();
                    for _ in 1..key {
                        s.push_str("X");
                    }
                    a.push(s);
                } else if key == 4 {
                    a.push("XL".to_string());
                } else if key == 5 {
                    a.push("L".to_string());
                } else if 9 > key && key >= 6 {
                    let mut s = "L".to_string();
                    for _ in 6..=key {
                        s.push_str("X");
                    }
                    a.push(s);
                } else if key == 9 {
                    a.push("XC".to_string());
                }
            } else if i == 2 {
                if 4 > key && key >= 1 {
                    let mut s = "C".to_string();
                    for _ in 1..key {
                        s.push_str("C");
                    }
                    a.push(s);
                } else if key == 4 {
                    a.push("CD".to_string());
                } else if key == 5 {
                    a.push("D".to_string());
                } else if 9 > key && key >= 6 {
                    let mut s = "D".to_string();
                    for _ in 6..=key {
                        s.push_str("C");
                    }
                    a.push(s);
                } else if key == 9 {
                    a.push("CM".to_string());
                }
            } else if i == 3 {
                if 4 > key && key >= 1 {
                    let mut s = "M".to_string();
                    for _ in 1..key {
                        s.push_str("M");
                    }
                    a.push(s);
                }
            }


            i += 1;
            n = ((num as i64) / (10 as i64).pow(i)) as i32;
        }
        let mut str = "".to_string();
        for s in a.iter().rev() {
            str.push_str(s);
        }

        return str;
    }
}
