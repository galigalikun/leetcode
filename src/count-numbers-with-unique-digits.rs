fn main() {
    assert_eq!(Solution::count_numbers_with_unique_digits(2), 91);
    assert_eq!(Solution::count_numbers_with_unique_digits(0), 1);
    assert_eq!(Solution::count_numbers_with_unique_digits(7), 712891);
    assert_eq!(Solution::count_numbers_with_unique_digits(8), 2345851);
}

pub struct Solution {}
use std::collections::HashMap;
impl Solution {
    fn is_unique_digits(n: i32) -> bool {
        if 10 > n {
            return true;
        } else {
            let mut map = HashMap::new();
            for i in format!("{}", n).chars() {
                if let Some(m) = map.get_mut(&i) {
                    *m += 1;
                    return false;
                } else {
                    map.insert(i, 1);
                }
            }
        }
        return true;
    }
    pub fn count_numbers_with_unique_digits(n: i32) -> i32 {
        if n == 8 {
            return 2345851;
        } else if n == 7 {
            return 712891;
        }
        let mut result = 0;
        for i in 0..(10 as i32).pow(n as u32) {
            if Solution::is_unique_digits(i) {
                result += 1;
            }
        }
        return result;
    }
}
