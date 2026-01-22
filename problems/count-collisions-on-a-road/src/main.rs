fn main() {
    assert_eq!(Solution::count_collisions("RLRSLL".to_string()), 5);
    assert_eq!(Solution::count_collisions("LLRR".to_string()), 0);
}

struct Solution;
impl Solution {
    pub fn count_collisions(directions: String) -> i32 {
        return directions
            .trim_matches(|c| c == 'L' || c == 'R')
            .chars()
            .filter(|&c| c != 'S')
            .count() as i32;
    }
}
