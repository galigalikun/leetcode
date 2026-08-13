fn main() {
    assert_eq!(Solution::longest_mountain(vec![2, 1, 4, 7, 3, 2, 5]), 5);
    assert_eq!(Solution::longest_mountain(vec![2, 2, 2]), 0);
}

struct Solution;
impl Solution {
    pub fn longest_mountain(arr: Vec<i32>) -> i32 {
        let n = arr.len();
        if n < 3 {
            return 0;
        }

        let mut ans = 0usize;
        let mut i = 1usize;

        while i + 1 < n {
            let is_peak = arr[i - 1] < arr[i] && arr[i] > arr[i + 1];
            if !is_peak {
                i += 1;
                continue;
            }

            let mut left = i;
            while left > 0 && arr[left - 1] < arr[left] {
                left -= 1;
            }

            let mut right = i;
            while right + 1 < n && arr[right] > arr[right + 1] {
                right += 1;
            }

            ans = ans.max(right - left + 1);
            i = right;
        }

        ans as i32
    }
}
