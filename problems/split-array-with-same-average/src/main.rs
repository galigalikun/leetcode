use std::collections::HashSet;

fn main() {
    assert_eq!(Solution::split_array_same_average(vec![1,2,3,4,5,6,7,8]), true);
    assert_eq!(Solution::split_array_same_average(vec![3,1]), false);
}

struct Solution{}
impl Solution {
    pub fn split_array_same_average(nums: Vec<i32>) -> bool {
        let n = nums.len();
        if n < 2 {
            return false;
        }

        let total_sum: i32 = nums.iter().sum();
        let possible_sizes: Vec<usize> = (1..=n / 2)
            .filter(|&k| (total_sum * k as i32) % n as i32 == 0)
            .collect();

        if possible_sizes.is_empty() {
            return false;
        }

        let left_len = n / 2;
        let right_len = n - left_len;
        let left = &nums[..left_len];
        let right = &nums[left_len..];

        let left_sums = Self::sums_by_count(left);
        let right_sums = Self::sums_by_count(right);

        for &k in &possible_sizes {
            let target = (total_sum * k as i32) / n as i32;
            let left_min = k.saturating_sub(right_len);
            let left_max = left_len.min(k);

            for left_count in left_min..=left_max {
                let right_count = k - left_count;
                if left_sums[left_count]
                    .iter()
                    .any(|&left_sum| right_sums[right_count].contains(&(target - left_sum)))
                {
                    return true;
                }
            }
        }

        false
    }

    fn sums_by_count(values: &[i32]) -> Vec<HashSet<i32>> {
        let len = values.len();
        let mut sums: Vec<HashSet<i32>> = (0..=len).map(|_| HashSet::new()).collect();
        sums[0].insert(0);

        for (idx, &value) in values.iter().enumerate() {
            for count in (1..=idx + 1).rev() {
                let additions: Vec<i32> = sums[count - 1].iter().map(|&s| s + value).collect();
                for sum in additions {
                    sums[count].insert(sum);
                }
            }
        }

        sums
    }
}
