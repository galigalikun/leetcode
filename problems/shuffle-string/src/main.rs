fn main() {
    assert_eq!(
        Solution::restore_string("codeleet".to_string(), vec![4, 5, 6, 7, 0, 2, 1, 3]),
        "leetcode"
    );
    assert_eq!(
        Solution::restore_string("abc".to_string(), vec![0, 1, 2]),
        "abc"
    );
}

struct Solution;
impl Solution {
    pub fn restore_string(s: String, indices: Vec<i32>) -> String {
        let mut a = s.chars().zip(indices).collect::<Vec<_>>();
        a.sort_by_key(|&(_, i)| i);
        return a.iter().map(|(c, _)| c).collect();
    }
}
