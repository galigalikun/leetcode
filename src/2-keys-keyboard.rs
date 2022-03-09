fn main() {
    assert_eq!(Solution::min_steps(3), 3);
    assert_eq!(Solution::min_steps(1), 0);
}

struct Solution{}
impl Solution {
    pub fn min_steps(n: i32) -> i32 {
        // A 1 -> 0
        // AA 2 -> 2
        // AAA 3 -> 3
        // AAAA 4 -> 4
        // AAAAA 5 -> 5
        // AAAAAA 6 -> 5
        // AAAAAAA 7 -> 7
        // AAAAAAAA 8 -> 6
        // AAAAAAAAA 9 -> 6
        // AAAAAAAAAA 10 -> 7
        // AAAAAAAAAAA 11 -> 11
        // AAAAAAAAAAAA 12 -> 7 3*3+3
        return 0;
    }
}
