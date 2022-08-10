fn main() {
    assert_eq!(Solution::rotated_digits(10), 4);
    assert_eq!(Solution::rotated_digits(1), 0);
    assert_eq!(Solution::rotated_digits(2), 1);
    assert_eq!(Solution::rotated_digits(857), 247);
}

struct Solution{}
impl Solution {
    fn rotated(n: i32) -> Option<i32> {
        return match n {
            0 => Some(0),
            1 => Some(1),
            2 => Some(5),
            3 => None,
            4 => None,
            5 => Some(2),
            6 => Some(9),
            7 => None,
            8 => Some(8),
            9 => Some(6),
            _ => None,
        }
    }
    pub fn rotated_digits(n: i32) -> i32 {
        // 10
        // range 1 2 3 4 5 6 7 8 9 10
        //   2     5 6     9
        // 10 -> 01 ng
        // 20
        // range 1 2 3 4 ... 20
        // 2 5 6 9 11 12(51 ng) 13(ng) 15 ( 5 1 ng )
        let mut ans = 0;
        'lp: for i in 1..=n {
            // format!("{}", i);
            let mut p = i/10;
            let q = i%10;
            let mut result = String::new();
            if let Some(a) = Solution::rotated(q) {
                result = format!("{}", a);
                while p > 0 {
                    let q = p%10;
                    p /= 10;
                    if let Some(b) = Solution::rotated(q) {
                        result = format!("{}{}", result, b);
                    } else {
                        continue 'lp;
                    }
                }
                if result.len() > 1 && &result[0..1] == "0" {
                    continue 'lp;
                }
                let c = result.parse::<i32>().unwrap();
                if i == c {
                    continue 'lp;
                }
                ans += 1;
            } else {
                continue 'lp;
            }

        }
        return ans;
    }
}
