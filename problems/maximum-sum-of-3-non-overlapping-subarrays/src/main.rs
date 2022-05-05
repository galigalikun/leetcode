fn main() {
    assert_eq!(
        Solution::max_sum_of_three_subarrays(vec![1, 2, 1, 2, 6, 7, 5, 1], 2),
        vec![0, 3, 5]
    );
    assert_eq!(
        Solution::max_sum_of_three_subarrays(vec![1, 2, 1, 2, 1, 2, 1, 2, 1], 2),
        vec![0, 2, 4]
    );
}

// https://www.tutorialcup.com/interview/array/maximum-sum-of-3-non-overlapping-subarrays.htm#:~:text=Algorithm%20for%20Maximum%20Sum%20of%203%20Non%2DOverlapping%20Subarrays,-The%20idea%20is&text=Traverse%20the%20array%20sum%20starting,right%5Bi%20%2B%20k%5D%5D.
struct Solution {}
impl Solution {
    pub fn max_sum_of_three_subarrays(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut sum = vec![0; nums.len() - k as usize + 1];
        let mut current = 0;
        for i in 0..k as usize {
            current += nums[i];
        }
        sum[0] = current;
        for i in k as usize..nums.len() {
            current -= nums[i - k as usize];
            current += nums[i];
            sum[i - k as usize + 1] = current;
        }
        let mut left = vec![0; sum.len()];
        let mut best = 0;
        for i in 0..sum.len() {
            if sum[i] > sum[best] {
                best = i;
            }
            left[i] = best;
        }
        best = sum.len() - 1;
        let mut right = vec![0; sum.len()];
        for i in (0..sum.len()).rev() {
            if sum[i] >= sum[best] {
                best = i;
            }
            right[i] = best;
        }
        let mut ans = vec![-1; 3];
        for i in k as usize..sum.len() - k as usize {
            let l = left[i - k as usize];
            let r = right[i + k as usize];
            if ans[0] == -1
                || sum[l] + sum[r] + sum[i]
                    > sum[ans[0] as usize] + sum[ans[1] as usize] + sum[ans[2] as usize]
            {
                ans[0] = l as i32;
                ans[1] = i as i32;
                ans[2] = r as i32;
            }
        }
        return ans;
    }
}
