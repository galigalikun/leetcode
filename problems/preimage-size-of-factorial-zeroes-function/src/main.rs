fn main() {
    assert_eq!(Solution::preimage_size_fzf(0), 5);
    assert_eq!(Solution::preimage_size_fzf(5), 0);
    assert_eq!(Solution::preimage_size_fzf(3), 5);
}

struct Solution{}
impl Solution {
    pub fn preimage_size_fzf(k: i32) -> i32 {
        // 0! -> 1
        // 1! -> 1
        // 2! -> 1+2 -> 3
        // 3! -> 1+2+3 -> 6
        // 4! -> 1+2+3+4 -> 10 (0)
        // 5! -> 1+2+3+4 -> 15 (5) k = 5??
        return 0;
    }
}
