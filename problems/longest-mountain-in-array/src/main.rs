fn main() {
    assert_eq!(Solution::longest_mountain(vec![2, 1, 4, 7, 3, 2, 5]), 5);
    assert_eq!(Solution::longest_mountain(vec![2, 2, 2]), 0);
}

struct Solution;
impl Solution {
    pub fn longest_mountain(arr: Vec<i32>) -> i32 {
        let mut ans = 0;
        for i in 0..arr.len() {
            let mut a = 0;
            for j in 0..i {
                if arr[j] < arr[i] {
                    a += 1;
                } else {
                    break;
                }
            }
            for j in i..arr.len() {
                if arr[i] > arr[j] {
                    a += 1;
                } else {
                    break;
                }
            }
            ans = std::cmp::max(ans, a);
        }
        return ans;
    }
}
