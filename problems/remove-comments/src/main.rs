fn main() {
    assert_eq!(Solution::remove_comments(vec!["/*Test program */".to_string(), "int main()".to_string(), "{ ".to_string(), "  // variable declaration ".to_string(), "int a, b, c;".to_string(), "/* This is a test".to_string(), "   multiline  ".to_string(), "   comment for ".to_string(), "   testing */".to_string(), "a = b + c;".to_string(), "}".to_string()]), vec!["int main()", "{ ", "  ", "int a, b, c;", "a = b + c;", "}"]);
    assert_eq!(Solution::remove_comments(vec!["a/*comment".to_string(), "line".to_string(), "more_comment*/b".to_string()]), vec!["ab"]);
}

struct Solution{}
impl Solution {
    pub fn remove_comments(source: Vec<String>) -> Vec<String> {
        let mut result = vec![];
        let mut in_comment = false;
        let mut in_string = false;
        let mut in_line = false;
        let mut line = String::new();
        for s in source {
            for c in s.chars() {
                println!("debug {} {} {} {}", in_comment, in_string, in_line, c);
                if in_comment {
                    if c == '*' && s.chars().nth(1) == Some('/') {
                        in_comment = false;
                    }
                } else if in_string {
                    if c == '"' {
                        in_string = false;
                    }
                } else if in_line {
                    if c == '\n' {
                        in_line = false;
                    }
                } else {
                    if c == '/' {
                        if s.chars().nth(1) == Some('/') {
                            in_line = true;
                        } else if s.chars().nth(1) == Some('*') {
                            in_comment = true;
                        } else {
                            line.push(c);
                        }
                    } else if c == '"' {
                        in_string = true;
                    } else {
                        line.push(c);
                    }
                }
            }
            if !in_comment && !in_string && !in_line {
                result.push(line);
                line = String::new();
            }
        }
        return result;
    }
}
