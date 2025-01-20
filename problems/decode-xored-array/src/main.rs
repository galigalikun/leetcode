fn main() {
    assert_eq!(Solution::decode(vec![1, 2, 3], 1), vec![1, 0, 2, 1]);
    assert_eq!(Solution::decode(vec![6, 2, 7, 3], 4), vec![4, 2, 0, 7, 4]);
}

struct Solution;
impl Solution {
    pub fn decode(encoded: Vec<i32>, first: i32) -> Vec<i32> {
        let mut result = vec![first];
        for i in 0..encoded.len() {
            result.push(encoded[i] ^ result[i]);
        }
        result
    }
}
