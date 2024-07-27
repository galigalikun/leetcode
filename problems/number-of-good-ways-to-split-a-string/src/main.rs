fn main() {
    assert_eq!(Solution::num_splits("aacaba".to_string()), 2);
    assert_eq!(Solution::num_splits("abcd".to_string()), 1);
}

struct Solution;
impl Solution {
    pub fn num_splits(s: String) -> i32 {
        let mut left = vec![0; s.len()];
        let mut right = vec![0; s.len()];
        let mut set = std::collections::HashSet::new();
        let mut count = 0;
        for i in 0..s.len() {
            set.insert(s.chars().nth(i).unwrap());
            left[i] = set.len();
        }
        set.clear();
        for i in (0..s.len()).rev() {
            set.insert(s.chars().nth(i).unwrap());
            right[i] = set.len();
        }
        for i in 0..s.len() - 1 {
            if left[i] == right[i + 1] {
                count += 1;
            }
        }
        count
    }
}
