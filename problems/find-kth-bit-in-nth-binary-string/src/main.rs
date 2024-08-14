fn main() {
    assert_eq!(Solution::find_kth_bit(3, 1), '0');
    assert_eq!(Solution::find_kth_bit(4, 11), '1');
}

struct Solution;
impl Solution {
    fn find_kth_bit_helper(n: i32, k: i32, reverse: bool) -> char {
        if n == 1 {
            return if reverse { '1' } else { '0' };
        }
        let len = 2_i32.pow(n as u32) - 1;
        if k == len / 2 + 1 {
            return if reverse { '0' } else { '1' };
        }
        if k < len / 2 + 1 {
            return Solution::find_kth_bit_helper(n - 1, k, reverse);
        }
        return Solution::find_kth_bit_helper(n - 1, len - k + 1, !reverse);
    }

    pub fn find_kth_bit(n: i32, k: i32) -> char {
        return Solution::find_kth_bit_helper(n, k, false);
    }
}
