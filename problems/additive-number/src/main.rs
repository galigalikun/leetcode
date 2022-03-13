fn main() {
    assert_eq!(Solution::is_additive_number("112358".to_string()), true);

    assert_eq!(Solution::is_additive_number("199100199".to_string()), true);

    assert_eq!(Solution::is_additive_number("0".to_string()), false);

    assert_eq!(Solution::is_additive_number("120122436".to_string()), false);
}

struct Solution {}
impl Solution {
    fn check_eq(prev1: &str, prev2: &str, num: String) -> bool {
        let (a, b) = if prev1.len() < prev2.len() {
            (prev2, prev1)
        } else {
            (prev1, prev2)
        };
        if num.len() as i32 - a.len() as i32 > 2 {
            return false;
        }
        if a.len() == 0 || b.len() == 0 {
            return false;
        }
        if (a.chars().nth(0) == Some('0') && a.len() > 1)
            || (b.chars().nth(0) == Some('0') && b.len() > 1)
        {
            return false;
        }

        let mut i = a.len() as i32 - 1;
        let mut j = b.len() as i32 - 1;
        let mut s = "".to_string();
        let mut carry = 0;

        while j >= 0 {
            let tmp = a.chars().nth(i as usize).unwrap() as i32 - 48
                + b.chars().nth(j as usize).unwrap() as i32
                - 48
                + carry;
            s = format!("{}{}", tmp % 10, s);
            carry = tmp / 10;
            i -= 1;
            j -= 1;
        }
        while i >= 0 {
            let tmp = a.chars().nth(i as usize).unwrap() as i32 - 48 + carry;
            s = format!("{}{}", tmp % 10, s);
            carry = tmp / 10;
            i -= 1;
        }
        if carry > 0 {
            s = format!("1{}", s);
        }
        return s == num;
    }
    fn helper(prev1: &str, prev2: &str, num: String, result: &mut bool) {
        if *result {
            return;
        }
        let sz = num.len();
        if sz == 0 {
            *result = true;
        } else {
            let mut p1 = prev1;
            let mut p2 = prev2;
            if p1.len() == 0 && sz > 2 {
                for i in 1..=sz - 2 {
                    p1 = &num[0..i];
                    Solution::helper(p1, p2, num[i..].to_string(), result);
                }
            }

            if p2.len() == 0 && sz > 1 {
                for i in 1..=sz - 1 {
                    p2 = &num[0..i];
                    Solution::helper(p1, p2, num[i..].to_string(), result);
                }
            }

            for i in 1..=sz {
                if Solution::check_eq(p1, p2, num[0..i].to_string()) {
                    Solution::helper(p2, &num[0..i], num[i..].to_string(), result);
                }
            }
        }
    }
    pub fn is_additive_number(num: String) -> bool {
        let mut result = false;
        Solution::helper("", "", num, &mut result);
        return result;
    }
}
