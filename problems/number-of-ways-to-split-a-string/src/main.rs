fn main() {
    assert_eq!(Solution::num_ways("10101".to_string()), 4);
    assert_eq!(Solution::num_ways("1001".to_string()), 0);
    assert_eq!(Solution::num_ways("0000".to_string()), 3);
}

struct Solution;
impl Solution {
    pub fn num_ways(s: String) -> i32 {
        let mut ones = 0;
        let n = s.len();
        let mut ones_count = vec![0; n];
        let mut total_ones = 0;
        for (i, c) in s.chars().enumerate() {
            if c == '1' {
                ones += 1;
                total_ones += 1;
            }
            ones_count[i] = ones;
        }
        if total_ones % 3 != 0 {
            return 0;
        }
        if total_ones == 0 {
            return ((n - 1) as i64 * (n - 2) as i64 / 2 % 1000000007) as i32;
        }
        let third = total_ones / 3;
        let mut first = 0;
        let mut second = 0;
        for (i, c) in s.chars().enumerate() {
            if ones_count[i] == third {
                first += 1;
            } else if ones_count[i] == 2 * third {
                second += 1;
            }
        }
        ((first as i64 * second as i64) % 1000000007) as i32
    }
}
