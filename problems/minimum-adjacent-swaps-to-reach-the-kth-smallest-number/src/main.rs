fn main() {
    assert_eq!(Solution::get_min_swaps("5489355142".to_string(), 4), 2);
    assert_eq!(Solution::get_min_swaps("11112".to_string(), 4), 4);
    assert_eq!(Solution::get_min_swaps("00123".to_string(), 1), 1);
}

struct Solution;
impl Solution {
    fn get_min_swaps_impl(num: String, k: i32) -> i32 {
        let mut num = num.chars().collect::<Vec<_>>();
        let n = num.len();
        let mut k = k;
        while k > 0 {
            let mut i = n - 1;
            while i > 0 && num[i] <= num[i - 1] {
                i -= 1;
            }
            if i == 0 {
                return -1;
            }
            let mut j = n - 1;
            while j >= i && num[j] <= num[i - 1] {
                j -= 1;
            }
            num.swap(i - 1, j);
            num[i..].reverse();
            k -= 1;
        }
        let mut swaps = 0;
        for i in (0..n).rev() {
            for j in (i + 1..n).rev() {
                if num[j] < num[i] {
                    swaps += 1;
                    num.swap(i, j);
                }
            }
        }
        swaps
    }
    pub fn get_min_swaps(num: String, k: i32) -> i32 {
        return Self::get_min_swaps_impl(num, k);
    }
}
