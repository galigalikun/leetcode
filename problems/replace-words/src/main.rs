fn main() {
    assert_eq!(
        Solution::replace_words(
            vec!["cat".to_string(), "bat".to_string(), "rat".to_string()],
            "the cattle was rattled by the battery".to_string()
        ),
        "the cat was rat by the bat"
    );
    assert_eq!(
        Solution::replace_words(
            vec!["a".to_string(), "b".to_string(), "c".to_string()],
            "aadsfasf absbs bbab cadsfafs".to_string()
        ),
        "a a b c"
    );
    assert_eq!(
        Solution::replace_words(
            vec![
                "a".to_string(),
                "aa".to_string(),
                "aaa".to_string(),
                "aaaa".to_string()
            ],
            "a aa a aaaa aaa aaa aaa aaaaaa bbb baba ababa".to_string()
        ),
        "a a a a a a a a bbb baba a"
    );
    assert_eq!(
        Solution::replace_words(
            vec![
                "catt".to_string(),
                "cat".to_string(),
                "bat".to_string(),
                "rat".to_string()
            ],
            "the cattle was rattled by the battery".to_string()
        ),
        "the cat was rat by the bat"
    );
}

struct Solution {}
impl Solution {
    pub fn replace_words(dictionary: Vec<String>, sentence: String) -> String {
        let mut dd = dictionary;
        dd.sort();
        let mut ans = String::new();
        for s in sentence.split(" ") {
            let mut p = s;
            for d in &dd {
                if s.len() >= d.len() && &s[0..d.len()] == d {
                    p = d;
                    break;
                }
            }
            ans = format!("{} {}", ans, p);
        }
        return ans.trim_start().to_string();
    }
}
