fn main() {
    assert_eq!(
        Solution::find_substring(
            "barfoothefoobarman".to_string(),
            vec!["foo".to_string(), "bar".to_string()]
        ),
        vec![0, 9]
    );

    assert_eq!(
        Solution::find_substring(
            "wordgoodgoodgoodbestword".to_string(),
            vec![
                "word".to_string(),
                "good".to_string(),
                "best".to_string(),
                "word".to_string()
            ]
        ),
        vec![]
    );

    assert_eq!(
        Solution::find_substring(
            "barfoofoobarthefoobarman".to_string(),
            vec!["bar".to_string(), "foo".to_string(), "the".to_string()]
        ),
        vec![6, 9, 12]
    );
}

pub struct Solution {}
use std::collections::HashMap;
impl Solution {
    pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
        // Number of a characters of a word in list L.
        let size_word = words[0].len();

        // Number of words present inside list L.
        let word_count = words.len();

        // Total characters present in list L.
        let size_l = size_word * word_count;

        // Resultant vector which stores indices.
        let mut res: Vec<i32> = vec![];
        let n = s.len();

        // If the total number of characters in list L
        // is more than length of string S itself.
        if size_l > n {
            return res;
        }

        // Map stores the words present in list L
        // against it's occurrences inside list L
        let mut map: HashMap<String, i32> = HashMap::new();

        for word in words {
            if let Some(m) = map.get_mut(&word) {
                *m += 1;
            } else {
                map.insert(word, 1);
            }
        }

        for i in 0..=n - size_l {
            let mut tmp = map.clone();
            let mut j = i;
            let mut count = word_count;

            // Traverse the substring
            while j < i + size_l {
                // Extract the word
                let word = &s[j..j + size_word];

                // If word not found or if frequency
                // of current word is more than required simply break.
                if !map.contains_key(word) || tmp.get(word) == Some(&0) {
                    break;
                }
                // Else decrement the count of word from hash_map
                else {
                    if let Some(m) = tmp.get_mut(word) {
                        *m -= 1;
                    }
                    count -= 1;
                }
                j += size_word;
            }
            // Store the starting index of that
            // substring when all the words in
            // the list are in substring
            if count == 0 {
                res.push(i as i32);
            }
        }
        return res;
    }
}
