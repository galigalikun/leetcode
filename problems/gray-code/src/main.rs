fn main() {
    assert_eq!(Solution::gray_code(2), vec![0, 1, 3, 2]);
    assert_eq!(Solution::gray_code(0), vec![0]);
}

pub struct Solution {}
impl Solution {
    fn to_gray_code(s: String) -> i32 {
        let mut result = Vec::new();
        for i in 0..s.as_str().chars().count() {
            if let Some(c) = match s.as_str().chars().nth(i) {
                Some('0') => {
                    if i == 0 {
                        Some('0')
                    } else {
                        s.as_str().chars().nth(i - 1)
                    }
                }
                Some('1') => {
                    if i == 0 {
                        Some('1')
                    } else if let Some('1') = s.as_str().chars().nth(i - 1) {
                        Some('0')
                    } else {
                        Some('1')
                    }
                }
                _ => None,
            } {
                result.push(c);
            }
        }
        return i32::from_str_radix(&result.iter().collect::<String>(), 2).unwrap();
    }
    pub fn gray_code(n: i32) -> Vec<i32> {
        // n 1 2
        // n 2 4
        // n 3 8
        // n 4 16
        // n 5 32
        let mut result = Vec::new();
        for i in 0..(2 as i32).pow(n as u32) {
            let s = format!("{:0>width$b}", i, width = n as usize);
            result.push(Solution::to_gray_code(s));
        }
        return result;
    }
}
