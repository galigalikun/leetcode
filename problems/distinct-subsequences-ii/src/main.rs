fn main() {
    assert_eq!(Solution::distinct_subseq_ii("abc".to_string()), 7);
    assert_eq!(Solution::distinct_subseq_ii("aba".to_string()), 6);
    assert_eq!(Solution::distinct_subseq_ii("aaa".to_string()), 3);
}

struct Solution;
impl Solution {
    pub fn distinct_subseq_ii(s: String) -> i32 {
        const MOD: i64 = 1_000_000_007;

        let mut total: i64 = 0;
        let mut end_with = [0_i64; 26];

        for b in s.bytes() {
            let index = (b - b'a') as usize;
            let new_count = (total + 1) % MOD;

            total = (total + new_count - end_with[index]) % MOD;
            if total < 0 {
                total += MOD;
            }

            end_with[index] = new_count;
        }

        total as i32
    }
}
