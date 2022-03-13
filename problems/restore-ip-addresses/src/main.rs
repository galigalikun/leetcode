fn main() {
    assert_eq!(
        Solution::restore_ip_addresses("25525511135".to_string()),
        vec!["255.255.11.135".to_string(), "255.255.111.35".to_string()]
    );
    assert_eq!(
        Solution::restore_ip_addresses("0000".to_string()),
        vec!["0.0.0.0".to_string()]
    );
    assert_eq!(
        Solution::restore_ip_addresses("1111".to_string()),
        vec!["1.1.1.1".to_string()]
    );
    assert_eq!(
        Solution::restore_ip_addresses("010010".to_string()),
        vec!["0.10.0.10".to_string(), "0.100.1.0".to_string()]
    );
    assert_eq!(
        Solution::restore_ip_addresses("101023".to_string()),
        vec![
            "1.0.10.23".to_string(),
            "1.0.102.3".to_string(),
            "10.1.0.23".to_string(),
            "10.10.2.3".to_string(),
            "101.0.2.3".to_string()
        ]
    );
}

struct Solution {}
// https://www.youtube.com/watch?v=nxBMEvLqDzY
impl Solution {
    fn restore(result: &mut Vec<String>, s: String, current: &str, field: usize) {
        if field == 4 && s.as_str().chars().count() == 0 {
            result.push(current[1..].to_string());
        } else if field == 4 || s.as_str().chars().count() == 0 {
            return;
        } else {
            Solution::restore(
                result,
                (&s[1..]).to_string(),
                &(current.to_string() + "." + &s[0..1]),
                field + 1,
            );
            if Some('0') != s.as_str().chars().nth(0) && s.as_str().chars().count() > 1 {
                Solution::restore(
                    result,
                    (&s[2..]).to_string(),
                    &(current.to_string() + "." + &s[0..2]),
                    field + 1,
                );
                if s.as_str().chars().count() > 2 && (&s[0..3]).parse::<u32>().unwrap() <= 255 {
                    Solution::restore(
                        result,
                        (&s[3..]).to_string(),
                        &(current.to_string() + "." + &s[0..3]),
                        field + 1,
                    );
                }
            }
        }
    }
    pub fn restore_ip_addresses(s: String) -> Vec<String> {
        // 12 xxx.xxx.xxx.xxx
        // 11 xxx.xxx.xxx.xx
        // 10 xxx.xxx.xxx.x xxx.xxx.xx.xx
        // 9  xxx.xxx.xx.x xxx.xx.xx.xx
        // 8  xxx.xxx.x.x xxx.xx.xx.x xx.xx.xx.xx
        // 7  xxx.xx.x.x xx.xx.xx.x
        // 6  xxx.x.x.x xx.xx.x.x
        // 5  xx.x.x.x
        // 4  x.x.x.x
        let result = &mut vec![];

        Solution::restore(result, s, "", 0);
        return result.to_vec();
    }
}
