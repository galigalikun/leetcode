fn main() {
    assert_eq!(
        Solution::multiply("2".to_string(), "3".to_string()),
        "6".to_string()
    );
    assert_eq!(
        Solution::multiply("123".to_string(), "456".to_string()),
        "56088".to_string()
    );
    assert_eq!(
        Solution::multiply("123456789".to_string(), "987654321".to_string()),
        "121932631112635269".to_string()
    );
    assert_eq!(
        Solution::multiply("9133".to_string(), "0".to_string()),
        "0".to_string()
    );
}

pub struct Solution {}
impl Solution {
    fn to_number(c: char) -> u16 {
        return match c {
            '0' => 0,
            '1' => 1,
            '2' => 2,
            '3' => 3,
            '4' => 4,
            '5' => 5,
            '6' => 6,
            '7' => 7,
            '8' => 8,
            '9' => 9,
            _ => panic!("not number"),
        };
    }

    pub fn multiply(num1: String, num2: String) -> String {
        if num1 == "0" || num2 == "0" {
            return "0".to_string();
        }
        let mut i = 0;
        let mut result = Vec::new();
        for n1 in num1.as_str().chars().rev() {
            let mut j = i;
            for n2 in num2.as_str().chars().rev() {
                let a1 = Solution::to_number(n1);
                let a2 = Solution::to_number(n2);
                // 58 * 3 = 8 * 3 + 10*5*3
                let r1 = (a1 * a2) % 10;
                let r2 = (a1 * a2) / 10;
                if result.len() > j {
                    let rr1 = (result[j] + r1) % 10;
                    let rr2 = (result[j] + r1) / 10;
                    result[j] = rr1;
                    if result.len() > j + 1 {
                        result[j + 1] += r2 + rr2;
                    } else {
                        result.push(r2 + rr2);
                    }
                } else {
                    result.push(r1);
                    result.push(r2);
                }
                j += 1;
            }
            i += 1;
        }
        let mut ss = "".to_string();
        let mut i = 0;
        for &d in result.iter() {
            if result.len() - 1 == i && d == 0 {
                i += 1;
                continue;
            }
            ss = format!("{}{}", d, ss);
            i += 1;
        }
        return ss;
    }
}
