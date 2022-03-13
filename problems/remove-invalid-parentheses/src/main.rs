fn main() {
    assert_eq!(
        Solution::remove_invalid_parentheses("()())()".to_string()),
        vec!["(())()", "()()()"]
    );

    assert_eq!(
        Solution::remove_invalid_parentheses("(a)())()".to_string()),
        vec!["(a())()", "(a)()()"]
    );

    assert_eq!(
        Solution::remove_invalid_parentheses(")(".to_string()),
        vec![""]
    );
}

struct Solution {}
// https://kennyzhuang.gitbooks.io/algorithms-collection/content/remove_invalid_parentheses1.html
impl Solution {
    fn helper(
        result: &mut Vec<String>,
        s: String,
        i: i32,
        rm_l: i32,
        rm_r: i32,
        open: i32,
        sb: &mut String,
    ) {
        if i == s.len() as i32 && rm_l == 0 && rm_r == 0 && open == 0 {
            if result.iter().find(|&ss| ss == sb) == None {
                result.push(sb.to_string());
            }
            return;
        }

        if i == s.len() as i32 || rm_l < 0 || rm_r < 0 || open < 0 {
            return;
        }

        let c = s.chars().nth(i as usize).unwrap();
        let len = sb.len();

        if c == '(' {
            Solution::helper(result, s.clone(), i + 1, rm_l - 1, rm_r, open, sb);
            sb.push(c);
            Solution::helper(result, s.clone(), i + 1, rm_l, rm_r, open + 1, sb);
        } else if c == ')' {
            Solution::helper(result, s.clone(), i + 1, rm_l, rm_r - 1, open, sb);
            sb.push(c);
            Solution::helper(result, s.clone(), i + 1, rm_l, rm_r, open - 1, sb);
        } else {
            sb.push(c);
            Solution::helper(result, s.clone(), i + 1, rm_l, rm_r, open, sb);
        }

        *sb = sb[0..len].to_string();
    }
    pub fn remove_invalid_parentheses(s: String) -> Vec<String> {
        if s.len() == 0 {
            return vec![];
        }
        let mut rm_l = 0;
        let mut rm_r = 0;
        for c in s.chars() {
            if c == '(' {
                rm_l += 1;
            } else if c == ')' {
                if rm_l != 0 {
                    rm_l -= 1;
                } else {
                    rm_r += 1;
                }
            }
        }

        let mut result = vec![];
        Solution::helper(&mut result, s, 0, rm_l, rm_r, 0, &mut String::new());

        return result;
    }
}
