fn main() {
    assert_eq!(
        Solution::brace_expansion_ii("{a,b}{c,{d,e}}".to_string()),
        vec!["ac", "ad", "ae", "bc", "bd", "be"]
    );
    assert_eq!(
        Solution::brace_expansion_ii("{{a,z},a{b,c},{ab,z}}".to_string()),
        vec!["a", "ab", "ac", "z"]
    );
}

struct Solution;
impl Solution {
    pub fn brace_expansion_ii(expression: String) -> Vec<String> {
        let mut stack = vec![];
        let mut cur = vec![];
        let mut cur_set = vec![];
        let mut cur_str = String::new();
        let mut is_str = false;
        let mut is_set = false;
        let mut is_set_str = false;
        for s in expression.split("{") {
            for t in s.split("}") {
                if t.is_empty() {
                    continue;
                }
                if is_str {
                    cur_str.push_str(t);
                    is_str = false;
                } else if is_set_str {
                    cur_set.push(t.to_string());
                    is_set_str = false;
                } else if is_set {
                    if t.contains(",") {
                        for u in t.split(",") {
                            cur_set.push(u.to_string());
                        }
                    } else {
                        cur_set.push(t.to_string());
                    }
                    is_set = false;
                } else {
                    if t.contains(",") {
                        for u in t.split(",") {
                            cur.push(u.to_string());
                        }
                    } else {
                        cur.push(t.to_string());
                    }
                }
            }
            if s.contains(",") {
                is_set = true;
            } else {
                is_str = true;
            }
        }
        if !cur_str.is_empty() {
            cur.push(cur_str);
        }
        if !cur_set.is_empty() {
            cur.push(cur_set.join(""));
        }
        for s in cur {
            if s.contains(",") {
                stack.push(Solution::brace_expansion_ii(s));
            } else {
                stack.push(vec![s]);
            }
        }
        if stack.len() == 1 {
            return stack.pop().unwrap();
        }
        let mut res = vec![];
        let mut cur = vec![];
        while !stack.is_empty() {
            let mut s = stack.pop().unwrap();
            if s.len() == 1 {
                cur.push(s.pop().unwrap());
            } else {
                let mut tmp = vec![];
                while !s.is_empty() {
                    let t = s.pop().unwrap();
                    if t.contains(",") {
                        tmp.push(t);
                    } else {
                        if cur.is_empty() {
                            cur.push(t);
                        } else {
                            let mut cur_tmp = vec![];
                            while !cur.is_empty() {
                                let mut u = cur.pop().unwrap();
                                u.push_str(&t);
                                cur_tmp.push(u);
                            }
                            cur = cur_tmp;
                        }
                    }
                }
                if !cur.is_empty() {
                    while !cur.is_empty() {
                        let u = cur.pop().unwrap();
                        while !tmp.is_empty() {
                            let mut v = tmp.pop().unwrap();
                            v.push_str(&u);
                            cur.push(v);
                        }
                    }
                } else {
                    cur = tmp;
                }
            }
        }
        while !cur.is_empty() {
            res.push(cur.pop().unwrap());
        }
        res.sort();
        res.dedup();
        return res;
    }
}
