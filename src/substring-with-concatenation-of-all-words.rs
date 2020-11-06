fn main() {
    assert_eq!(
        Solution::find_substring(
            "barfoothefoobarman".to_string(),
            vec!["foo".to_string(), "bar".to_string()]
        ),
        vec![0, 9]
    );
}

pub struct Solution {}
impl Solution {
    fn aaaa(s: String, words: &mut Vec<String>) {
        if let Some(w) = words.pop() {
            let mut ss = s.as_str();
            while let Some(p) = ss.find(&w) {
                ss = &ss[p + w.len()..];
                println!(
                    "aaaa {} {} index:{} {}",
                    w,
                    p,
                    s.len() - ss.len() - w.len(),
                    ss
                );
            }
        }
    }
    pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
        // let mut word = words;

        // while let Some(w) = word.pop() {
        //     let mut ss = s.as_str();
        //     while let Some(p) = ss.find(&w) {
        //         ss = &ss[p+w.len()..];
        //         println!("debug {} {} index:{} {}", w, p, s.len()-ss.len()-w.len(), ss);
        //         Solution::aaaa(s, &mut word);
        //     }
        // }
        let mut hits = Vec::new();
        for w in words {
            let mut ss = s.as_str();
            let mut hit = Vec::new();
            while let Some(p) = ss.find(&w) {
                ss = &ss[p + w.len()..];
                // Solution::aaaa("ss".to_string(), vec!["".to_string()]);
                // + 18 - 3  - 12 - 3 =
                // 18 - 0 - 12 - 0
                hit.push(s.len() - ss.len() - w.len());
                println!(
                    "debug {} {} index:{} {}",
                    w,
                    p,
                    s.len() - ss.len() - w.len(),
                    ss
                );
            }
            hits.push(hit);
        }
        println!("debug {:?}", hits);
        return vec![];
    }
}
