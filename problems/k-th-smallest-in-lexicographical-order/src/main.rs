fn main() {
    assert_eq!(Solution::find_kth_number(13, 2), 10);
    assert_eq!(Solution::find_kth_number(1, 1), 1);
    assert_eq!(Solution::find_kth_number(681692778, 351251360), 416126219);
}

struct Solution {}
// https://dreamume.medium.com/leetcode-440-k-th-smallest-in-lexicographical-order-6e5331d67e76
impl Solution {
    pub fn find_kth_number(n: i32, k: i32) -> i32 {
        if n < k || k < 1 {
            return 0;
        } else if n < 10 {
            return k;
        }
        let mut result = 1;
        let mut kk = k as i64 - 1;
        while kk > 0 {
            let mut step = 0;
            let mut first = result;
            let mut last = result + 1;
            while first <= n as i64 {
                step += std::cmp::min(n as i64 + 1, last) - first;
                first *= 10;
                last *= 10;
            }

            if step <= kk {
                result += 1;
                kk -= step;
            } else {
                result *= 10;
                kk -= 1;
            }
        }

        return result as i32;
    }
}
