use std::collections::HashMap;

fn main() {
    assert_eq!(
        Solution::shortest_completing_word(
            "1s3 PSt".to_string(),
            vec![
                "step".to_string(),
                "steps".to_string(),
                "stripe".to_string(),
                "stepple".to_string()
            ]
        ),
        "steps"
    );
    assert_eq!(
        Solution::shortest_completing_word(
            "1s3 456".to_string(),
            vec![
                "looks".to_string(),
                "pest".to_string(),
                "stew".to_string(),
                "show".to_string()
            ]
        ),
        "pest"
    );
}

struct Solution {}
impl Solution {
    pub fn shortest_completing_word(license_plate: String, words: Vec<String>) -> String {
        let mut dic = HashMap::new();
        for p in license_plate.chars() {
            match p {
               '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
               }
                _ => {
                    *dic.entry(p).or_insert(0) += 1;
                },
            }
        }

        let mut n = 0;
        let mut ans = String::new();
        for w in words {
        let mut m = 0;
            let mut map = dic.clone();
            for c in w.chars() {
                if let Some(v) = map.get_mut(&c) {
                    if *v > 0 {
                        *v -= 1;
                        m+=1;
                    }
                }
            }
            if m > n {
                n = m;
                ans = w;
            }

        }
        return ans;
    }
}
