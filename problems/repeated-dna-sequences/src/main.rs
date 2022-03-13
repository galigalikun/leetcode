fn main() {
    assert_eq!(
        Solution::find_repeated_dna_sequences("AAAAACCCCCAAAAACCCCCCAAAAAGGGTTT".to_string()),
        vec!["AAAAACCCCC", "CCCCCAAAAA"]
    );

    assert_eq!(
        Solution::find_repeated_dna_sequences("AAAAAAAAAAAAA".to_string()),
        vec!["AAAAAAAAAA"]
    );
}

struct Solution {}
use std::collections::HashMap;
impl Solution {
    pub fn find_repeated_dna_sequences(s: String) -> Vec<String> {
        let n = s.len();
        if n <= 10 {
            return vec![];
        }
        let mut result = HashMap::new();
        for i in 0..=n - 10 {
            let sub = &s[i..i + 10];
            if let Some(r) = result.get_mut(sub) {
                *r += 1;
            } else {
                result.insert(sub, 1);
            }
        }
        return result
            .iter()
            .filter(|(_, &v)| v > 1)
            .map(|(&k, _)| k.to_string())
            .collect::<Vec<_>>();
    }
}
