fn main() {
    println!("number-of-matching-subsequences");
}

struct Solution;
impl Solution {
    pub fn num_matching_subseq(s: String, words: Vec<String>) -> i32 {
        let mut positions = vec![Vec::<usize>::new(); 26];
        for (index, byte) in s.bytes().enumerate() {
            positions[(byte - b'a') as usize].push(index);
        }

        words
            .iter()
            .filter(|word| {
                let mut current_index = 0usize;

                for byte in word.bytes() {
                    let indices = &positions[(byte - b'a') as usize];
                    let offset = indices.partition_point(|&i| i < current_index);

                    if offset == indices.len() {
                        return false;
                    }

                    current_index = indices[offset] + 1;
                }

                true
            })
            .count() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_case_one() {
        let s = "abcde".to_string();
        let words = vec!["a", "bb", "acd", "ace"]
            .into_iter()
            .map(String::from)
            .collect();

        assert_eq!(Solution::num_matching_subseq(s, words), 3);
    }

    #[test]
    fn example_case_two() {
        let s = "dsahjpjauf".to_string();
        let words = vec!["ahjpjau", "ja", "ahbwzgqnuk", "tnmlanowax"]
            .into_iter()
            .map(String::from)
            .collect();

        assert_eq!(Solution::num_matching_subseq(s, words), 2);
    }

    #[test]
    fn counts_duplicates() {
        let s = "aaaaa".to_string();
        let words = vec!["a", "a", "aa", "aaa", "aaaaaa"]
            .into_iter()
            .map(String::from)
            .collect();

        assert_eq!(Solution::num_matching_subseq(s, words), 4);
    }
}
