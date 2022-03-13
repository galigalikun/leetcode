fn main() {
    assert_eq!(Solution::fraction_to_decimal(1, 2), "0.5".to_string());
    assert_eq!(Solution::fraction_to_decimal(2, 1), "2".to_string());
    assert_eq!(Solution::fraction_to_decimal(2, 3), "0.(6)".to_string());
    assert_eq!(
        Solution::fraction_to_decimal(-1, -2147483648),
        "0.0000000004656612873077392578125".to_string()
    );
}

// https://www.programcreek.com/2014/03/leetcode-fraction-to-recurring-decimal-java/
struct Solution {}
use std::collections::HashMap;
impl Solution {
    pub fn fraction_to_decimal(numerator: i32, denominator: i32) -> String {
        if numerator == 0 {
            return "0".to_string();
        }
        if denominator == 0 {
            return "0".to_string();
        }
        let mut result = "".to_string();
        if (numerator < 0) ^ (denominator < 0) {
            result += "-";
        }
        let num = (numerator as i64).abs();
        let den = (denominator as i64).abs();
        let mut res = num / den;
        result = format!("{}{}", result, res);
        let mut remainder = (num % den) * 10;
        if remainder == 0 {
            return result;
        }

        let mut map = HashMap::new();
        result += ".";

        while remainder != 0 {
            if let Some(&m) = map.get(&remainder) {
                let part1 = &result[0..m];
                let part2 = &result[m..];
                result = format!("{}({})", part1, part2);
                return result;
            }

            map.insert(remainder, result.len());
            res = remainder / den;
            result = format!("{}{}", result, res);
            remainder = (remainder % den) * 10;
        }

        return result;
    }
}
