fn main() {
    // assert_eq!(Solution::is_match("aa".to_string(), "a".to_string()), false);
    // assert_eq!(Solution::is_match("aa".to_string(), "a*".to_string()), true);
    // assert_eq!(Solution::is_match("ab".to_string(), ".*".to_string()), true);
    // assert_eq!(Solution::is_match("aab".to_string(), "c*a*b".to_string()), true);
    // assert_eq!(Solution::is_match("mississippi".to_string(), "mis*is*p*.".to_string()), false);
    // assert_eq!(Solution::is_match("ab".to_string(), ".*c".to_string()), false);
    // assert_eq!(Solution::is_match("aaa".to_string(), "a*a".to_string()), true);
    // assert_eq!(Solution::is_match("aaa".to_string(), "ab*a".to_string()), false);
    // assert_eq!(Solution::is_match("a".to_string(), "ab*".to_string()), true);
    assert_eq!(
        Solution::is_match("bbbba".to_string(), ".*a*a".to_string()),
        true
    );
    // assert_eq!(Solution::is_match("ab".to_string(), ".*..".to_string()), true);
    // assert_eq!(Solution::is_match("a".to_string(), ".*..a*".to_string()), false);
}

pub struct Solution {}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: char,
    pub all: bool,
    pub next: Option<Box<ListNode>>,
}

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let ss = s.as_str();
        let mut prev = '\0';
        let mut idx = 0;
        let mut m_idx = 0;
        let mut matches = Vec::new();
        for a in p.as_str().chars() {
            match a {
                '*' => {
                    matches.remove(matches.len() - 1);
                    matches.push(ListNode {
                        val: prev,
                        all: true,
                        next: None,
                    });
                }
                b => {
                    matches.push(ListNode {
                        val: b,
                        all: false,
                        next: None,
                    });
                    prev = b;
                }
            }
        }
        /*
        .*
        a*
        a
        */
        let mut a: Option<Box<ListNode>> = None;
        for n in matches.iter().rev() {
            a = Some(Box::new(ListNode {
                val: n.val,
                all: n.all,
                next: a,
            }));
        }

        println!("debug {:?}", a);

        loop {
            if let Some(m) = a {
                match ss.chars().nth(idx) {
                    Some(c) => {
                        if m.val == '.' && m.all {
                            idx += 1;
                            if let Some(n) = m.next {
                                if n.all {}
                            }
                        }
                    }
                    None => {}
                }
                a = m.next;
            } else {
                break;
            }
        }

        let max_count = matches.iter().count();

        for m in matches {
            m_idx += 1;
            match ss.chars().nth(idx) {
                Some(c) => {
                    if m.val == '.' && m.all {
                        idx += 1;
                        println!("debug {}", idx);
                        if let Some(n) = m.next {
                            while let Some(b) = ss.chars().nth(idx) {
                                if n == '.' || b == n {
                                    break;
                                }
                                idx += 1;
                            }
                            println!("debug {}", idx);
                        } else {
                            while let Some(_) = ss.chars().nth(idx) {
                                idx += 1;
                            }
                        }
                    } else if m.val == '.' && !m.all {
                        idx += 1;
                    } else if m.val != c && m.all {
                        // idx += 1;
                    } else if m.val == c && !m.all {
                        idx += 1;
                    } else if m.val == c && m.all {
                        idx += 1;
                        while let Some(cc) = ss.chars().nth(idx) {
                            if cc != m.val {
                                break;
                            }
                            idx += 1;
                        }
                        if ss.chars().count() == idx && max_count > m_idx {
                            idx -= 1;
                        }
                    } else if m.val != c && !m.all {
                        return false;
                    }
                }
                None => {
                    if !m.all {
                        return false;
                        // println!("debug aaa {}", aaa);
                        // if aaa == 0 {
                        //     return false;
                        // }
                        // for i in 1..=aaa {
                        //     match ss.chars().nth(idx-i) {
                        //         Some(_) => {
                        //             if m.val == '.' {
                        //                 aaa-=1;
                        //             }
                        //         },
                        //         None => {
                        //             return false;
                        //         }
                        //     }
                        // }
                        // println!("debug aaa {}", aaa);
                        // if aaa == 0 && max_count == m_idx {
                        //     return true;
                        // }
                    }
                }
            }
        }

        println!("result {} {}", ss.chars().count(), idx);
        return ss.chars().count() == idx;
    }
}
