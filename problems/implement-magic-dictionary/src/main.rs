use std::collections::HashMap;

struct MagicDictionary {
    map:HashMap<usize,Vec<String>>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MagicDictionary {

    fn new() -> Self {
        MagicDictionary {
            map:HashMap::new(),
        }
    }

    fn build_dict(&mut self, dictionary: Vec<String>) {
        for word in dictionary {
            if let Some(m) = self.map.get_mut(&word.len()) {
                m.push(word);
            } else {
                self.map.insert(word.len(),vec![word]);
            }
        }
    }

    fn search(&self, search_word: String) -> bool {
        if let Some(m) = self.map.get(&search_word.len()) {
            for word in m {
                if word.len() != search_word.len() {
                    continue;
                }
                let mut diff = 0;
                for (i,c) in word.chars().enumerate() {
                    if c != search_word.chars().nth(i).unwrap() {
                        diff += 1;
                    }
                }
                if diff == 1 {
                    return true;
                }
            }
        }
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
    let mut obj = MagicDictionary::new();
    obj.build_dict(vec!["hello".to_string(), "leetcode".to_string()]);
    assert_eq!(obj.search("hello".to_string()), false);
    assert_eq!(obj.search("hhllo".to_string()), true);
    assert_eq!(obj.search("hell".to_string()), false);
    assert_eq!(obj.search("leetcoded".to_string()), false);
}
