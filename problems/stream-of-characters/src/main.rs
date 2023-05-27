struct StreamChecker {
    words: Vec<String>,
    query: Vec<char>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl StreamChecker {
    fn new(words: Vec<String>) -> Self {
        StreamChecker {
            words: words,
            query: Vec::new(),
        }
    }

    fn query(&mut self, letter: char) -> bool {
        self.query.push(letter);
        for word in self.words.iter() {
            if word.len() > self.query.len() {
                continue;
            }
            let mut matched = true;
            for i in 0..word.len() {
                if word.chars().nth(word.len() - i - 1).unwrap()
                    != self.query[self.query.len() - i - 1]
                {
                    matched = false;
                    break;
                }
            }
            if matched {
                return true;
            }
        }
        return false;
    }
}

/**
 * Your StreamChecker object will be instantiated and called as such:
 * let obj = StreamChecker::new(words);
 * let ret_1: bool = obj.query(letter);
 */
fn main() {
    let mut obj = StreamChecker::new(vec!["cd".to_string(), "f".to_string(), "kl".to_string()]);
    assert_eq!(obj.query('a'), false);
    assert_eq!(obj.query('b'), false);
    assert_eq!(obj.query('c'), false);
    assert_eq!(obj.query('d'), true);
    assert_eq!(obj.query('e'), false);
    assert_eq!(obj.query('f'), true);
    assert_eq!(obj.query('j'), false);
    assert_eq!(obj.query('h'), false);
    assert_eq!(obj.query('i'), false);
    assert_eq!(obj.query('j'), false);
    assert_eq!(obj.query('k'), false);
    assert_eq!(obj.query('l'), true);
}
