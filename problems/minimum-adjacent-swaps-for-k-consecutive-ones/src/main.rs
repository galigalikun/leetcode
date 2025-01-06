fn main() {
    assert_eq!(Solution::min_moves(vec![1, 0, 0, 1, 0, 1], 2), 1);
    assert_eq!(Solution::min_moves(vec![1, 0, 0, 0, 0, 0, 1, 1], 3), 5);
    assert_eq!(Solution::min_moves(vec![1, 1, 0, 1], 2), 0);
}

struct Solution;
impl Solution {
    pub fn min_moves(nums: Vec<i32>, k: i32) -> i32 {
        let mut ones = vec![];
        for i in 0..nums.len() {
            if nums[i] == 1 {
                ones.push(i as i32 - ones.len() as i32);
            }
        }
        let n = ones.len();
        let mut pre = vec![0; n + 1];
        for i in 0..n {
            pre[i + 1] = pre[i] + ones[i];
        }
        let mut ans = i32::MAX;
        for i in 0..n {
            let mid = i as i32 + k / 2;
            let mut sum = 0;
            for j in 0..n {
                sum += (ones[j] - ones[i]).abs();
                if k % 2 == 0 {
                    sum -= (ones[j] - ones[i]).abs();
                }
                if j < mid as usize {
                    sum += ones[i] * (j as i32 - i as i32) - (pre[j + 1] - pre[i + 1]);
                } else {
                    sum += (pre[j + 1] - pre[i + 1]) - ones[i] * (j as i32 - i as i32);
                }
                if j as i32 >= k - 1 {
                    ans = ans.min(sum);
                }
            }
        }
        ans
    }
}
