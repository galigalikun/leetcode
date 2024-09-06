fn main() {
    assert_eq!(
        Solution::find_length_of_shortest_subarray(vec![1, 2, 3, 10, 4, 2, 3, 5]),
        3
    );
    assert_eq!(
        Solution::find_length_of_shortest_subarray(vec![5, 4, 3, 2, 1]),
        4
    );
    assert_eq!(Solution::find_length_of_shortest_subarray(vec![1, 2, 3]), 0);
}

struct Solution;
impl Solution {
    pub fn find_length_of_shortest_subarray(arr: Vec<i32>) -> i32 {
        let n = arr.len();
        let mut left = 0;
        for i in 1..n {
            if arr[i] < arr[i - 1] {
                left = i;
                break;
            }
        }
        if left == n - 1 {
            return 0;
        }
        let mut right = n - 1;
        for i in (0..n - 1).rev() {
            if arr[i] > arr[i + 1] {
                right = i;
                break;
            }
        }
        let mut ans = std::cmp::min(n - left - 1, right);
        let mut i = 0;
        let mut j = right;
        while i <= left && j < n {
            if arr[i] <= arr[j] {
                ans = std::cmp::min(ans, j - i - 1);
                i += 1;
            } else {
                j += 1;
            }
        }
        ans as i32
    }
}
