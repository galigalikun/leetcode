fn main() {
    assert_eq!(Solution::remove_comments(vec!["/*Test program */".to_string(), "int main()".to_string(), "{ ".to_string(), "  // variable declaration ".to_string(), "int a, b, c;".to_string(), "/* This is a test".to_string(), "   multiline  ".to_string(), "   comment for ".to_string(), "   testing */".to_string(), "a = b + c;".to_string(), "}".to_string()]), vec!["int main()", "{ ", "  ", "int a, b, c;", "a = b + c;", "}"]);
    assert_eq!(Solution::remove_comments(vec!["a/*comment".to_string(), "line".to_string(), "more_comment*/b".to_string()]), vec!["ab"]);
}

struct Solution{}
impl Solution {
    pub fn remove_comments(source: Vec<String>) -> Vec<String> {
        let mut result = Vec::new();
        let mut in_block_comment = false;
        let mut current_line = String::new();

        for line in source {
            let bytes = line.as_bytes();
            let mut i = 0;

            while i < bytes.len() {
                if in_block_comment {
                    if i + 1 < bytes.len() && bytes[i] == b'*' && bytes[i + 1] == b'/' {
                        in_block_comment = false;
                        i += 2;
                    } else {
                        i += 1;
                    }
                    continue;
                }

                if i + 1 < bytes.len() && bytes[i] == b'/' && bytes[i + 1] == b'/' {
                    break;
                }

                if i + 1 < bytes.len() && bytes[i] == b'/' && bytes[i + 1] == b'*' {
                    in_block_comment = true;
                    i += 2;
                    continue;
                }

                current_line.push(bytes[i] as char);
                i += 1;
            }

            if !in_block_comment && !current_line.is_empty() {
                result.push(std::mem::take(&mut current_line));
            }
        }

        result
    }
}
