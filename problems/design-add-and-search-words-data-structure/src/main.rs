struct WordDictionary {
    data: Vec<String>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl WordDictionary {
    /** Initialize your data structure here. */
    fn new() -> Self {
        WordDictionary { data: vec![] }
    }

    fn add_word(&mut self, word: String) {
        self.data.push(word);
    }

    fn search(&self, word: String) -> bool {
        for s in &self.data {
            if s == &word {
                return true;
            } else if s.len() == word.len() {
                let mut i = 0;
                for c in word.as_str().chars() {
                    if c == '.' {
                    } else if Some(c) == s.as_str().chars().nth(i) {
                    } else {
                        break;
                    }
                    i += 1;
                }
                if i == s.len() {
                    return true;
                }
            }
        }
        return false;
    }
}

/**
 * Your WordDictionary object will be instantiated and called as such:
 * let obj = WordDictionary::new();
 * obj.add_word(word);
 * let ret_2: bool = obj.search(word);
 */
fn main() {
    let mut obj = WordDictionary::new();
    obj.add_word("bad".to_string());
    obj.add_word("dad".to_string());
    obj.add_word("mad".to_string());

    assert_eq!(obj.search("pad".to_string()), false);
    assert_eq!(obj.search("bad".to_string()), true);
    assert_eq!(obj.search(".ad".to_string()), true);
    assert_eq!(obj.search("b..".to_string()), true);
}
