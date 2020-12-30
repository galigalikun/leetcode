fn main() {
    assert_eq!(
        Solution::min_window("ADOBECODEBANC".to_string(), "ABC".to_string()),
        "BANC".to_string()
    );
    assert_eq!(
        Solution::min_window("a".to_string(), "a".to_string()),
        "a".to_string()
    );
    assert_eq!(
        Solution::min_window("a".to_string(), "aa".to_string()),
        "".to_string()
    );
    assert_eq!(
        Solution::min_window("bdab".to_string(), "ab".to_string()),
        "ab".to_string()
    );
}

pub struct Solution {}
impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        let (mut start, mut end) = (0, 0);
        let mut distance: Option<(usize, usize)> = None;
        let mut max = 0;
        for i in 1..=t.as_str().chars().count() {
            max += i;
        }
        let mut hit: i32 = -1;
        for p in 0..s.as_str().chars().count() {
            for i in 0..t.as_str().chars().count() {
                if s.as_str().chars().nth(p) == t.as_str().chars().nth(i) {
                    if hit == -1 {
                        start = p;
                        hit = (i + 1) as i32;
                    } else {
                        hit += (i + 1) as i32;
                    }

                    if hit == max as i32 {
                        end = p;
                        println!("debug {} {}", start, end);
                        if distance == None {
                            distance = Some((start, end));
                        } else if let Some((ss, ee)) = distance {
                            if (ee - ss) > (end - start) {
                                distance = Some((start, end));
                            }
                        }
                        hit = (i + 1) as i32;
                        start = p;
                    }
                    break;
                }
            }
        }
        if let Some((ss, ee)) = distance {
            return (&s[ss..=ee]).to_string();
        }
        return "".to_string();
    }
}
