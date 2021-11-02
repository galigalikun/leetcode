fn main() {
    assert_eq!(Solution::count_range_sum(vec![-2, 5, -1], -2, 2), 3);

    assert_eq!(Solution::count_range_sum(vec![0], 0, 0), 1);

    assert_eq!(
        Solution::count_range_sum(vec![-2147483647, 0, -2147483647, 2147483647], -564, 3864),
        3
    );
    assert_eq!(Solution::count_range_sum(vec![0, -3, -3, 1, 1, 2], 3, 5), 2);
}

pub struct Solution {}
// https://liumingzhang.gitbooks.io/google-questions/content/count_of_range_sum.html
impl Solution {
    fn merge(sum: &mut Vec<i64>, start: i32, end: i32, lower: i64, upper: i64) -> i32 {
        if end - start <= 1 {
            return 0;
        }
        let mid = start + (end - start) / 2;
        let mut count = Solution::merge(sum, start, mid, lower, upper)
            + Solution::merge(sum, mid, end, lower, upper);

        let mut j = mid;
        let mut k = mid;
        let mut t = mid;
        let mut r = 0;

        let mut cache = vec![0; end as usize - start as usize];
        for i in start..mid {
            while j < end && sum[j as usize] - sum[i as usize] < lower {
                j += 1;
            }
            while k < end && sum[k as usize] - sum[i as usize] <= upper {
                k += 1;
            }
            while t < end && sum[t as usize] < sum[i as usize] {
                cache[r] = sum[t as usize];
                r += 1;
                t += 1;
            }
            cache[r] = sum[i as usize];
            r += 1;
            count += k - j;
        }

        for i in 0..(t - start) as usize {
            sum[i + start as usize] = cache[i];
        }
        return count;
    }
    pub fn count_range_sum(nums: Vec<i32>, lower: i32, upper: i32) -> i32 {
        let n = nums.len();
        let mut sum = vec![0; n + 1];
        for i in 0..n {
            sum[i + 1] = sum[i] + nums[i] as i64;
        }

        return Solution::merge(&mut sum, 0, 1 + n as i32, lower as i64, upper as i64);
    }
}
