fn main() {
    assert_eq!(
        Solution::are_sentences_similar("My name is Haley".to_string(), "My Haley".to_string()),
        true
    );
    assert_eq!(
        Solution::are_sentences_similar("of".to_string(), "A lot of words".to_string()),
        false
    );
    assert_eq!(
        Solution::are_sentences_similar("Eating right now".to_string(), "Eating".to_string()),
        true
    );
}

struct Solution;
impl Solution {
    fn is_similar(words1: &[&str], words2: &[&str]) -> bool {
        let mut i = 0;
        let mut j = words1.len() - 1;
        let mut k = words2.len() - 1;

        while i < words1.len() && j < words1.len() && k < words2.len() {
            if words1[i] != words2[k] {
                break;
            }
            i += 1;
            k -= 1;
        }

        while j > 0 && k > 0 {
            if words1[j] != words2[k] {
                break;
            }
            j -= 1;
            k -= 1;
        }

        return i == j + 1 || k == 0;
    }
    pub fn are_sentences_similar(sentence1: String, sentence2: String) -> bool {
        let words1: Vec<&str> = sentence1.split_whitespace().collect();
        let words2: Vec<&str> = sentence2.split_whitespace().collect();
        return (words1.len() == words2.len() && words1 == words2)
            || (words1.len() > words2.len() && Self::is_similar(&words1, &words2))
            || (words1.len() < words2.len() && Self::is_similar(&words2, &words1));
    }
}
