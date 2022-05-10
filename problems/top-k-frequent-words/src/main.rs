fn main() {
    assert_eq!(
        Solution::top_k_frequent(
            vec![
                "i".to_string(),
                "love".to_string(),
                "leetcode".to_string(),
                "i".to_string(),
                "love".to_string(),
                "coding".to_string()
            ],
            2
        ),
        vec!["i", "love"]
    );
    assert_eq!(
        Solution::top_k_frequent(
            vec![
                "the".to_string(),
                "day".to_string(),
                "is".to_string(),
                "sunny".to_string(),
                "the".to_string(),
                "the".to_string(),
                "the".to_string(),
                "sunny".to_string(),
                "is".to_string(),
                "is".to_string()
            ],
            4
        ),
        vec!["the", "is", "sunny", "day"]
    );
    assert_eq!(
        Solution::top_k_frequent(
            vec!["a".to_string(), "aa".to_string(), "aaa".to_string()],
            1
        ),
        vec!["a"]
    );
    assert_eq!(
        Solution::top_k_frequent(
            vec![
                "i".to_string(),
                "love".to_string(),
                "leetcode".to_string(),
                "i".to_string(),
                "love".to_string(),
                "coding".to_string()
            ],
            3
        ),
        vec!["i", "love", "coding"]
    );
    assert_eq!(
        Solution::top_k_frequent(
            vec![
                "plpaboutit".to_string(),
                "jnoqzdute".to_string(),    // 1
                "sfvkdqf".to_string(),
                "mjc".to_string(),
                "nkpllqzjzp".to_string(),
                "foqqenbey".to_string(),
                "ssnanizsav".to_string(),
                "nkpllqzjzp".to_string(),
                "sfvkdqf".to_string(),
                "isnjmy".to_string(),
                "pnqsz".to_string(),
                "hhqpvvt".to_string(),  // 2
                "fvvdtpnzx".to_string(),    // hit
                "jkqonvenhx".to_string(),
                "cyxwlef".to_string(),
                "hhqpvvt".to_string(),
                "fvvdtpnzx".to_string(),
                "plpaboutit".to_string(),
                "sfvkdqf".to_string(),
                "mjc".to_string(),
                "fvvdtpnzx".to_string(),
                "bwumsj".to_string(),
                "foqqenbey".to_string(),
                "isnjmy".to_string(),
                "nkpllqzjzp".to_string(),
                "hhqpvvt".to_string(),
                "foqqenbey".to_string(),
                "fvvdtpnzx".to_string(),
                "bwumsj".to_string(),
                "hhqpvvt".to_string(),
                "fvvdtpnzx".to_string(),
                "jkqonvenhx".to_string(),
                "jnoqzdute".to_string(),
                "foqqenbey".to_string(),
                "jnoqzdute".to_string(),
                "foqqenbey".to_string(),
                "hhqpvvt".to_string(),
                "ssnanizsav".to_string(),
                "mjc".to_string(),
                "foqqenbey".to_string(),
                "bwumsj".to_string(),
                "ssnanizsav".to_string(),
                "fvvdtpnzx".to_string(),
                "nkpllqzjzp".to_string(),
                "jkqonvenhx".to_string(),
                "hhqpvvt".to_string(),
                "mjc".to_string(),
                "isnjmy".to_string(),
                "bwumsj".to_string(),
                "pnqsz".to_string(),
                "hhqpvvt".to_string(),
                "nkpllqzjzp".to_string(),
                "jnoqzdute".to_string(),
                "pnqsz".to_string(),
                "nkpllqzjzp".to_string(),
                "jnoqzdute".to_string(),
                "foqqenbey".to_string(),
                "nkpllqzjzp".to_string(),
                "hhqpvvt".to_string(),
                "fvvdtpnzx".to_string(),
                "plpaboutit".to_string(),
                "jnoqzdute".to_string(),
                "sfvkdqf".to_string(),
                "fvvdtpnzx".to_string(),
                "jkqonvenhx".to_string(),
                "jnoqzdute".to_string(),
                "nkpllqzjzp".to_string(),
                "jnoqzdute".to_string(),
                "fvvdtpnzx".to_string(),
                "jkqonvenhx".to_string(),
                "hhqpvvt".to_string(),
                "isnjmy".to_string(),
                "jkqonvenhx".to_string(),
                "ssnanizsav".to_string(),
                "jnoqzdute".to_string(),
                "jkqonvenhx".to_string(),
                "fvvdtpnzx".to_string(),
                "hhqpvvt".to_string(),
                "bwumsj".to_string(),
                "nkpllqzjzp".to_string(),
                "bwumsj".to_string(),
                "jkqonvenhx".to_string(),
                "jnoqzdute".to_string(),
                "pnqsz".to_string(),
                "foqqenbey".to_string(),
                "sfvkdqf".to_string(),
                "sfvkdqf".to_string()
            ],
            1
        ),
        vec!["fvvdtpnzx"]
    )
}

struct Solution {}
use std::collections::HashMap;
impl Solution {
    pub fn top_k_frequent(words: Vec<String>, k: i32) -> Vec<String> {
        let mut map = HashMap::new();
        for word in words.clone() {
            *map.entry(word).or_insert(0) += 1;
        }
        let mut vec = vec![];
        for (word, count) in map {
            vec.push((count, word));
        }

        vec.sort_by(|a, b| a.cmp(&b));
        vec.sort_by(|a, b| b.0.cmp(&a.0));
        return vec
            .iter()
            .map(|x| &x.1)
            .take(k as usize)
            .map(|x| x.to_string())
            .collect::<Vec<_>>();
    }
}
