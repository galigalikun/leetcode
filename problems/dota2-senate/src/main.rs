fn main() {
    assert_eq!(Solution::predict_party_victory("RD".to_string()), "Radiant");
    assert_eq!(Solution::predict_party_victory("RDD".to_string()), "Dire");
}

struct Solution{}
impl Solution {
    pub fn predict_party_victory(senate: String) -> String {
        return senate;
    }
}
