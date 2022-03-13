fn main() {
    assert_eq!(
        Solution::count_segments("Hello, my name is John".to_string()),
        5
    );
    assert_eq!(Solution::count_segments("Hello".to_string()), 1);
    assert_eq!(
        Solution::count_segments("love live! mu'sic forever".to_string()),
        4
    );
    assert_eq!(Solution::count_segments("".to_string()), 0);
    assert_eq!(Solution::count_segments("                ".to_string()), 0);
    assert_eq!(
        Solution::count_segments(", , , ,        a, eaefa".to_string()),
        6
    );
}

struct Solution {}
impl Solution {
    pub fn count_segments(s: String) -> i32 {
        return std::cmp::min(
            s.trim().split(" ").filter(|x| !x.is_empty()).count(),
            s.trim().len(),
        ) as i32;
    }
}
