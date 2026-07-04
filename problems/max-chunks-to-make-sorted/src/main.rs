fn main() {
    assert_eq!(Solution::max_chunks_to_sorted(vec![4, 3, 2, 1, 0]), 1);
    assert_eq!(Solution::max_chunks_to_sorted(vec![1, 0, 2, 3, 4]), 4);
}

struct Solution {}
impl Solution {
    pub fn max_chunks_to_sorted(arr: Vec<i32>) -> i32 {
        let mut max_seen = -1;
        let mut chunks = 0;

        for (i, &num) in arr.iter().enumerate() {
            if num > max_seen {
                max_seen = num;
            }

            if max_seen == i as i32 {
                chunks += 1;
            }
        }

        chunks
    }
}
