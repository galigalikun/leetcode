fn main() {
    assert_eq!(
        Solution::is_valid("<DIV>This is the first line <![CDATA[<div>]]></DIV>".to_string()),
        true
    );
    assert_eq!(
        Solution::is_valid("<DIV>>>  ![cdata[]] <![CDATA[<div>]>]]>]]>>]</DIV>".to_string()),
        true
    );
    assert_eq!(
        Solution::is_valid("<A>  <B> </A>   </B>".to_string()),
        false
    );
    assert_eq!(
        Solution::is_valid("<A><B><![CDATA[<B></A>]]></B></A>".to_string()),
        true
    );
}

//https://www.programmerall.com/article/4471386632/
struct Solution {}
impl Solution {
    pub fn is_valid(code: String) -> bool {
        let mut i = 0;
        let mut stack = Vec::new();
        while i < code.len() {
            if i > 0 && stack.is_empty() {
                return false;
            }
            if i + 9 < code.len() && &code[i..i + 9] == "<![CDATA[" {
                let j = i + 9;
                if let Some(p) = code[j..].find("]]>") {
                    i = j + p;
                } else {
                    return false;
                }
                i += 2;
            } else if i + 2 < code.len() && &code[i..i + 2] == "</" {
                let j = i + 2;
                if let Some(p) = code[j..].find(">") {
                    i = j + p;
                } else {
                    return false;
                }
                let tag = &code[j..i];
                if stack.is_empty() || stack.first() != Some(&tag) {
                    return false;
                }
                stack.pop();
            } else if i + 1 < code.len() && &code[i..i + 1] == "<" {
                let j = i + 1;
                if let Some(p) = code[j..].find(">") {
                    i = j + p;
                } else {
                    return false;
                }
                if i == j || i - j > 9 {
                    return false;
                }
                for k in j..i {
                    if code.chars().nth(k) < Some('A') || code.chars().nth(k) > Some('Z') {
                        return false;
                    }
                }
                let tag = &code[j..i];
                stack.push(tag);
            }
            i += 1;
        }
        return stack.is_empty();
    }
}
