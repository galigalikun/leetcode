fn main() {
    assert_eq!(Solution::reverse_parentheses("(abcd)".to_string()), "dcba");
    assert_eq!(
        Solution::reverse_parentheses("(u(love)i)".to_string()),
        "iloveu"
    );
    assert_eq!(
        Solution::reverse_parentheses("(ed(et(oc))el)".to_string()),
        "leetcode"
    );
}

struct Solution;
impl Solution {
    pub fn reverse_parentheses(s: String) -> String {
        let mut stack = vec![];
        let mut s = s.chars().collect::<Vec<char>>();
        for i in 0..s.len() {
            if s[i] == '(' {
                stack.push(i);
            } else if s[i] == ')' {
                let mut j = stack.pop().unwrap();
                let mut k = i;
                while j < k {
                    s.swap(j, k);
                    j += 1;
                    k -= 1;
                }
            }
        }
        s.retain(|&x| x != '(' && x != ')');

        return s.iter().collect::<String>();
    }
}
