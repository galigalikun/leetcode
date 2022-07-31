fn main() {
    assert_eq!(Solution::kth_grammar(1, 1), 0);
    assert_eq!(Solution::kth_grammar(2, 1), 0);
    assert_eq!(Solution::kth_grammar(2, 2), 1);
}

struct Solution{}
impl Solution {
    pub fn kth_grammar(n: i32, k: i32) -> i32 {
        let mut ans = 0;
        let mut a = vec![vec![0];n as usize];
        for i in 0..k {
            
        }
        return ans;
    }
}
