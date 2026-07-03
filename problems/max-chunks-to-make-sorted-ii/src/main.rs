fn main() {
    assert_eq!(Solution::max_chunks_to_sorted(vec![5,4,3,2,1]), 1);
    assert_eq!(Solution::max_chunks_to_sorted(vec![2,1,3,4,4]), 4);
}

struct Solution{}
impl Solution {
    pub fn max_chunks_to_sorted(arr: Vec<i32>) -> i32 {
        use std::collections::HashMap;

        let mut sorted = arr.clone();
        sorted.sort_unstable();

        let mut delta: HashMap<i32, i32> = HashMap::new();
        let mut chunks = 0;

        for (a, b) in arr.into_iter().zip(sorted.into_iter()) {
            *delta.entry(a).or_insert(0) += 1;
            if delta.get(&a) == Some(&0) {
                delta.remove(&a);
            }

            *delta.entry(b).or_insert(0) -= 1;
            if delta.get(&b) == Some(&0) {
                delta.remove(&b);
            }

            if delta.is_empty() {
                chunks += 1;
            }
        }

        chunks
    }
}
