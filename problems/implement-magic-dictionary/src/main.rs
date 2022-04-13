struct MagicDictionary {

}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MagicDictionary {

    fn new() -> Self {
        MagicDictionary {  }
    }

    fn build_dict(&self, dictionary: Vec<String>) {

    }

    fn search(&self, search_word: String) -> bool {
        return false;
    }
}

/**
 * Your MagicDictionary object will be instantiated and called as such:
 * let obj = MagicDictionary::new();
 * obj.build_dict(dictionary);
 * let ret_2: bool = obj.search(searchWord);
 */
fn main() {
    let obj = MagicDictionary::new();
    obj.build_dict(vec!["hello".to_string(), "leetcode".to_string()]);
    assert_eq!(obj.search("hello".to_string()), false);
    assert_eq!(obj.search("hhllo".to_string()), true);
    assert_eq!(obj.search("hell".to_string()), false);
    assert_eq!(obj.search("leetcoded".to_string()), false);
}
