fn main() {
    assert_eq!(Solution::check_valid_string("()".to_string()), true);
    assert_eq!(Solution::check_valid_string("(*)".to_string()), true);
    assert_eq!(Solution::check_valid_string("(*))".to_string()), true);
}

// https://ttzztt.gitbooks.io/lc/content/valid-parenthesis-string.html
struct Solution {}
impl Solution {
    fn helper(result: &mut Vec<Vec<i32>>, s: String, start: usize, end: usize) -> bool {
        if start > end {
            return true;
        }
        if start == end {
            result[start][end] = if s.chars().nth(start) == Some('*') {
                1
            } else {
                0
            };
            return result[start][end] == 1;
        }
        if result[start][end] >= 0 {
            return result[start][end] == 1;
        }
        if (s.chars().nth(start) == Some('(') || s.chars().nth(start) == Some('*'))
            && (s.chars().nth(end) == Some(')') || s.chars().nth(end) == Some('*'))
        {
            if Solution::helper(result, s.clone(), start + 1, end - 1) {
                result[start][end] = 1;
                return true;
            }
        }

        for i in start..end {
            if Solution::helper(result, s.clone(), start, i)
                && Solution::helper(result, s.clone(), i + 1, end)
            {
                result[start][end] = 1;
                return true;
            }
        }

        result[start][end] = 0;
        return false;
    }
    pub fn check_valid_string(s: String) -> bool {
        let mut result = vec![vec![-1; s.len()]; s.len()];
        return Solution::helper(&mut result, s.clone(), 0, s.len() - 1);
    }
}
