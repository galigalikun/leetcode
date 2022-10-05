fn main() {
    assert_eq!(Solution::push_dominoes("RR.L".to_string()), "RR.L");
    assert_eq!(Solution::push_dominoes(".L.R...LR..L..".to_string()), "LL.RR.LLRRLL..");
}

struct Solution{}
impl Solution {
    pub fn push_dominoes(dominoes: String) -> String {
        return dominoes;
    }
}
