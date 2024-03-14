fn main() {
    assert_eq!(Solution::get_kth(12, 15, 2), 13);
    assert_eq!(Solution::get_kth(7, 11, 4), 7);
}

struct Solution;
impl Solution {
    fn power(mut n: i32) -> i32 {
        let mut count = 0;
        while n != 1 {
            if n % 2 == 0 {
                n /= 2;
            } else {
                n = 3 * n + 1;
            }
            count += 1;
        }
        return count;
    }
    pub fn get_kth(lo: i32, hi: i32, k: i32) -> i32 {
        let mut arr = vec![];
        for i in lo..=hi {
            arr.push((i, Solution::power(i)));
        }
        arr.sort_by(|a, b| a.1.cmp(&b.1));
        return arr[k as usize - 1].0;
    }
}
