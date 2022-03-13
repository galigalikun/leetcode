fn main() {
    assert_eq!(
        Solution::partition("aab".to_string()),
        vec![
            vec!["a".to_string(), "a".to_string(), "b".to_string()],
            vec!["aa".to_string(), "b".to_string()]
        ]
    );
    assert_eq!(
        Solution::partition("a".to_string()),
        vec![vec!["a".to_string()]]
    );
    assert_eq!(
        Solution::partition("efe".to_string()),
        vec![
            vec!["e".to_string(), "f".to_string(), "e".to_string()],
            vec!["efe".to_string()]
        ]
    );
    assert_eq!(
        Solution::partition("cdd".to_string()),
        vec![
            vec!["c".to_string(), "d".to_string(), "d".to_string()],
            vec!["c".to_string(), "dd".to_string()]
        ]
    );
    assert_eq!(
        Solution::partition("fff".to_string()),
        vec![
            vec!["f".to_string(), "f".to_string(), "f".to_string()],
            vec!["f".to_string(), "ff".to_string()],
            vec!["ff".to_string(), "f".to_string()],
            vec!["fff".to_string()]
        ]
    );
}

// https://programmerstart.com/article/9406351389/
struct Solution {}
impl Solution {
    fn helper(result: &mut Vec<Vec<String>>, s: String, lst: &mut Vec<String>, pos: usize) {
        let n = s.len();
        if n == pos {
            result.push(lst.to_vec());
            return;
        }
        for i in pos + 1..=n {
            let substr = &s[pos..i];
            let mut x = 0;
            let mut y = substr.len() - 1;
            let mut b = true;
            loop {
                if x >= y {
                    break;
                }
                if substr.chars().nth(x) != substr.chars().nth(y) {
                    b = false;
                    break;
                }
                x += 1;
                y -= 1;
            }
            if b {
                lst.push(substr.to_string());
                Solution::helper(result, s.clone(), lst, i);
                lst.remove(lst.len() - 1);
            }
        }
    }
    pub fn partition(s: String) -> Vec<Vec<String>> {
        let mut result: Vec<Vec<String>> = vec![];
        Solution::helper(&mut result, s, &mut vec![], 0);
        return result;
    }
}
