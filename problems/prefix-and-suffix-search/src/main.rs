struct WordFilter {
    words: Vec<String>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl WordFilter {
    fn new(words: Vec<String>) -> Self {
        WordFilter { words: words }
    }

    fn f(&self, pref: String, suff: String) -> i32 {
        for (i, w) in self.words.iter().enumerate() {
            if w[0..pref.len()] == pref && w[w.len() - suff.len()..] == suff {
                return i as i32;
            }
        }
        return -1;
    }
}

/**
 * Your WordFilter object will be instantiated and called as such:
 * let obj = WordFilter::new(words);
 * let ret_1: i32 = obj.f(pref, suff);
 */
fn main() {
    let obj = WordFilter::new(vec!["apple".to_string()]);
    assert_eq!(obj.f("a".to_string(), "e".to_string()), 0);
    assert_eq!(obj.f("b".to_string(), "e".to_string()), -1);
    {
        let obj = WordFilter::new(vec!["abbba".to_string(), "abba".to_string()]);
        assert_eq!(obj.f("ab".to_string(), "ba".to_string()), 1);
    }
}
