fn main() {
    assert_eq!(Solution::decode(vec![3, 1]), vec![1, 2, 3]);
    assert_eq!(Solution::decode(vec![6, 5, 4, 6]), vec![2, 4, 1, 5, 3]);
}

struct Solution;
impl Solution {
    pub fn decode(encoded: Vec<i32>) -> Vec<i32> {
        let mut res = vec![0; encoded.len() + 1];
        let mut total = 0;
        for i in 1..=encoded.len() + 1 {
            total ^= i as i32;
        }
        let mut odd = 0;
        for i in (1..encoded.len()).step_by(2) {
            odd ^= encoded[i];
        }
        res[0] = total ^ odd;
        for i in 0..encoded.len() {
            res[i + 1] = res[i] ^ encoded[i];
        }
        res
    }
}
