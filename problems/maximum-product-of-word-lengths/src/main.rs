fn main() {
    assert_eq!(
        Solution::max_product(vec![
            "abcw".to_string(),
            "baz".to_string(),
            "foo".to_string(),
            "bar".to_string(),
            "xtfn".to_string(),
            "abcdef".to_string()
        ]),
        16
    );

    assert_eq!(
        Solution::max_product(vec![
            "a".to_string(),
            "ab".to_string(),
            "abc".to_string(),
            "d".to_string(),
            "cd".to_string(),
            "bcd".to_string(),
            "abcd".to_string()
        ]),
        4
    );

    assert_eq!(
        Solution::max_product(vec![
            "a".to_string(),
            "aa".to_string(),
            "aaa".to_string(),
            "aaaa".to_string()
        ]),
        0
    );
}

struct Solution {}
impl Solution {
    pub fn max_product(words: Vec<String>) -> i32 {
        let mut result = 0;
        for i in 0..words.len() {
            for j in i + 1..words.len() {
                let mut b = false;
                for c in words[i].chars() {
                    if words[j].chars().find(|&x| c == x) != None {
                        b = true;
                        break;
                    }
                }
                if !b {
                    result = std::cmp::max(result, words[i].len() * words[j].len());
                }
            }
        }
        return result as i32;
    }
}
