fn main() {
    assert_eq!(
        20,
        Solution::three_sum_multi(vec![1, 1, 2, 2, 3, 3, 4, 4, 5, 5], 8)
    );
    assert_eq!(12, Solution::three_sum_multi(vec![1, 1, 2, 2, 2, 2], 5));
}
struct Solution;
impl Solution {
    pub fn three_sum_multi(arr: Vec<i32>, target: i32) -> i32 {
        let mut ans = 0;
        for i in 0..arr.len() - 2 {
            for j in i + 1..arr.len() - 1 {
                for k in j + 2..arr.len() {
                    if arr[i] + arr[j] + arr[k] == target {
                        ans += 1;
                    }
                }
            }
        }
        return ans;
    }
}
