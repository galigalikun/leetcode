use rand::{thread_rng, Rng};
use std::collections::HashMap;
struct Codec {
    rng: rand::rngs::ThreadRng,
    map: HashMap<String, String>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Codec {
    fn new() -> Self {
        Codec{
            rng:thread_rng(),
            map: HashMap::new(),
        }
    }

    // Encodes a URL to a shortened URL.
    fn encode(&mut self, long_url: String) -> String {
        loop {
            let key: [char; 6] = self.rng.gen();
            let s = key.iter().collect::<String>();
            if !self.map.contains_key(&s) {
                self.map.insert(s.clone(), long_url);
                return s;
            }
        }
    }

    // Decodes a shortened URL to its original URL.
    fn decode(&mut self, short_url: String) -> String {
        if let Some(m) = self.map.get(&short_url) {
            return m.to_string();
        }
        return "".to_string();
    }
}

/**
 * Your Codec object will be instantiated and called as such:
 * let obj = Codec::new();
 * let s: String = obj.encode(strs);
 * let ans: VecVec<String> = obj.decode(s);
 */
fn main() {
    let mut obj = Codec::new();
    let s = obj.encode("https://leetcode.com/problems/design-tinyurl".to_string());
    assert_eq!(obj.decode(s), "https://leetcode.com/problems/design-tinyurl");
}
