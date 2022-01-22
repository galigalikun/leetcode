fn main() {
    assert_eq!(Solution::fraction_addition("-1/2+1/2".to_string()), "0/1");
    assert_eq!(
        Solution::fraction_addition("-1/2+1/2+1/3".to_string()),
        "1/3"
    );
    assert_eq!(Solution::fraction_addition("1/3-1/2".to_string()), "-1/6");
}

// https://www.programmerall.com/article/71901939087/
struct Solution {}
impl Solution {
    fn helper(a: i64, b: i64) -> i64 {
        return if b == 0 {
            a
        } else {
            Solution::helper(b, a % b)
        };
    }
    pub fn fraction_addition(expression: String) -> String {
        let (mut n, mut d) = (0, 1);
        let mut p = 0;
        let new_expression = if expression.chars().nth(0) != Some('-') {
            format!("+{}", expression)
        } else {
            expression
        };
        while p < new_expression.len() {
            let mut p1 = p + 1;
            while new_expression.chars().nth(p1) != Some('/') {
                p1 += 1;
            }
            let mut p2 = p1 + 1;
            while p2 < new_expression.len()
                && new_expression.chars().nth(p2) != Some('+')
                && new_expression.chars().nth(p2) != Some('-')
            {
                p2 += 1;
            }

            let nn = &new_expression[p + 1..p1].parse::<i64>().unwrap();
            let dd = &new_expression[p1 + 1..p2].parse::<i64>().unwrap();
            let gcd = Solution::helper(d, *dd);
            n = n * dd / gcd
                + (if new_expression.chars().nth(p) == Some('-') {
                    -1
                } else {
                    1
                }) * nn
                    * d
                    / gcd;
            d *= dd / gcd;
            p = p2;
        }
        let gcd = Solution::helper(n.abs(), d);
        return format!("{}/{}", n / gcd, d / gcd);
    }
}
