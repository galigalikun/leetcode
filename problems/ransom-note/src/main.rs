fn main() {
    assert_eq!(
        Solution::can_construct("a".to_string(), "b".to_string()),
        false
    );
    assert_eq!(
        Solution::can_construct("aa".to_string(), "ab".to_string()),
        false
    );
    assert_eq!(
        Solution::can_construct("aa".to_string(), "aab".to_string()),
        true
    );
    assert_eq!(
        Solution::can_construct("aab".to_string(), "baa".to_string()),
        true
    );
}

pub struct Solution {}
impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        let mut ransom_vec = ransom_note.chars().collect::<Vec<_>>();
        for c in magazine.chars() {
            if let Some(p) = ransom_vec.iter().position(|&x| x == c) {
                ransom_vec.remove(p);
            }
        }
        if ransom_vec.len() == 0 {
            return true;
        }
        return false;
    }
}
