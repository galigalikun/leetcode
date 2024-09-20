fn main() {
    assert_eq!(
        Solution::max_sum_range_query(vec![1, 2, 3, 4, 5], vec![vec![1, 3], vec![0, 1]]),
        19
    );
    assert_eq!(
        Solution::max_sum_range_query(vec![1, 2, 3, 4, 5, 6], vec![vec![0, 1]]),
        11
    );
    assert_eq!(
        Solution::max_sum_range_query(
            vec![1, 2, 3, 4, 5, 10],
            vec![vec![0, 2], vec![1, 3], vec![1, 1]]
        ),
        47
    );
}

struct Solution;
impl Solution {
    pub fn max_sum_range_query(nums: Vec<i32>, requests: Vec<Vec<i32>>) -> i32 {
        let mut count = vec![0; nums.len()];
        for r in requests {
            count[r[0] as usize] += 1;
            if r[1] < nums.len() as i32 - 1 {
                count[r[1] as usize + 1] -= 1;
            }
        }
        for i in 1..nums.len() {
            count[i] += count[i - 1];
        }
        let mut nums = nums;
        nums.sort_unstable();
        count.sort_unstable();
        let mut ans = 0;
        for i in 0..nums.len() {
            ans += nums[i] * count[i];
        }
        ans % 1000000007
    }
}
