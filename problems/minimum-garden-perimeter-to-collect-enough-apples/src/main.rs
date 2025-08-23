fn main() {
    assert_eq!(Solution::minimum_perimeter(1), 8);
    assert_eq!(Solution::minimum_perimeter(13), 16);
    assert_eq!(Solution::minimum_perimeter(1000000000), 5040);
}

struct Solution;
impl Solution {
    pub fn minimum_perimeter(needed_apples: i64) -> i64 {
        return 8 * ((needed_apples + 3) / 4);
    }
}
