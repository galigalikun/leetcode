fn main() {
    assert_eq!(
        Solution::add_binary("11".to_string(), "1".to_string()),
        "100".to_string()
    );
    assert_eq!(
        Solution::add_binary("1010".to_string(), "1011".to_string()),
        "10101".to_string()
    );
    assert_eq!(
        Solution::add_binary("0".to_string(), "0".to_string()),
        "0".to_string()
    );
    assert_eq!(
        Solution::add_binary("100".to_string(), "110010".to_string()),
        "110110".to_string()
    );
}

struct Solution {}
impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let (cs1, cs2) = if a.as_str().chars().count() == b.as_str().chars().count() {
            (a, b)
        } else if a.as_str().chars().count() > b.as_str().chars().count() {
            let width = a.as_str().chars().count() - b.as_str().chars().count();
            (a, format!("{:0width$}{}", 0, b, width = width))
        } else {
            let width = b.as_str().chars().count() - a.as_str().chars().count();
            (format!("{:0width$}{}", 0, a, width = width), b)
        };

        let mut advanced = false;
        let mut result = Vec::new();
        for n in (0..cs1.as_str().chars().count()).rev() {
            if let Some(c1) = cs1.as_str().chars().nth(n) {
                if let Some(c2) = cs2.as_str().chars().nth(n) {
                    if c1 == '1' && c2 == '1' {
                        if advanced {
                            result.push('1');
                        } else {
                            result.push('0');
                        }
                        advanced = true;
                    } else if c1 == '1' || c2 == '1' {
                        if advanced {
                            result.push('0');
                        } else {
                            result.push('1');
                        }
                    } else {
                        if advanced {
                            result.push('1');
                        } else {
                            result.push('0');
                        }
                        advanced = false;
                    }
                }
            }
        }

        if advanced {
            result.push('1');
        }

        return result.iter().rev().collect();
    }
}
