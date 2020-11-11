fn main() {
    // assert_eq!(
    //     Solution::multiply("2".to_string(), "3".to_string()),
    //     "6".to_string()
    // );
    // assert_eq!(
    //     Solution::multiply("123".to_string(), "456".to_string()),
    //     "56088".to_string()
    // );
    assert_eq!(
        Solution::multiply("123456789".to_string(), "987654321".to_string()),
        "121932631112635269".to_string()
    );
}

pub struct Solution {}
impl Solution {
    fn toNumber(c: char) -> u16 {
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
        let mut num: u128 = 0;
        let mut i = 0;
        let mut result = Vec::new();
        for n1 in num1.as_str().chars().rev() {
            let mut j = 0;
            for n2 in num2.as_str().chars().rev() {
                let a1 = Solution::toNumber(n1);
                let a2 = Solution::toNumber(n2);
                // 58 * 3 = 8 * 3 + 10*5*3
                let r1 = (a1 * a2) % 10;
                let r2 = (a1 * a2) / 10;
                // [1, 2] 0, 1
                if result.len() > (i + j) {
                    result[i + j] += r1;
                    if result.len() > (i + j + 1) {
                        result[i + j + 1] += r2;
                    } else if r2 != 0 {
                        result.push(r2);
                    }
                } else {
                    result.push(r1);
                    if r2 != 0 {
                        result.push(r2);
                    }
                }

                // num += (a1 as u128) * (a2 as u128) * (10 as u128).pow(i + j);
                j += 1;
            }
            i += 1;
        }
        let mut ss = "".to_string();
        let mut aaa = 0;
        println!("debug result {:?}", result);
        for &d in result.iter() {
            println!("debug {}", d);
            if (aaa + d) > 9 {
                println!("debug aaa {} {} {}", aaa, d, aaa + d % 10);
                // 10 /10 -> 1
                // 1
                // 6
                ss = format!("{}{}", aaa + d % 10, ss);
                aaa = (aaa + d) / 10;
                println!("debug aaa tugi {} {} {}", aaa, d, aaa + d % 10);
            } else {
                ss = format!("{}{}", aaa + d, ss);
                aaa = 0;
            }
        }
        println!("debug ss {}", ss);
        return format!("{}", num);
    }
}
