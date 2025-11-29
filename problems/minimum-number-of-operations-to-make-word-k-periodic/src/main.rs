fn main() {
    assert_eq!(
        Solution::minimum_operations_to_make_k_periodic("leetcodeleet".to_string(), 4),
        1
    );
    assert_eq!(
        Solution::minimum_operations_to_make_k_periodic("leetcoleet".to_string(), 2),
        3
    );
}

struct Solution;
impl Solution {
    pub fn minimum_operations_to_make_k_periodic(word: String, k: i32) -> i32 {
        let n = word.len();
        let k = k as usize;
        let mut ans = 0;
        let bytes = word.as_bytes();
        for i in 0..k {
            let mut count = vec![0; 26];
            let mut total = 0;
            let mut j = i;
            while j < n {
                count[(bytes[j] - b'a') as usize] += 1;
                total += 1;
                j += k;
            }
            let max_freq = *count.iter().max().unwrap();
            ans += total - max_freq;
        }
        ans
    }
}
