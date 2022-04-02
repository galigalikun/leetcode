fn main() {
    assert_eq!(Solution::construct_array(3, 1), vec![1, 2, 3]);
    assert_eq!(Solution::construct_array(3, 2), vec![1, 3, 2]);
    assert_eq!(Solution::construct_array(5, 2), vec![1, 3, 2, 4, 5]);
}

// https://twchen.gitbook.io/leetcode/array/beautiful-arrangement-ii
struct Solution {}
impl Solution {
    pub fn construct_array(n: i32, k: i32) -> Vec<i32> {
        let mut ans = vec![0; n as usize];
        for i in 0..k / 2 {
            ans[2 * i as usize] = i + 1;
            ans[2 * i as usize + 1] = n - i;
        }
        let mut a = (0, 0);
        if k % 2 == 1 {
            a.0 = 1;
            a.1 = k / 2 + 1;
        } else {
            a.0 = n;
            a.1 = n - k / 2;
        }
        for i in k / 2 * 2..n {
            ans[i as usize] = a.1;
            a.1 += a.0;
        }
        return ans;
    }
}
