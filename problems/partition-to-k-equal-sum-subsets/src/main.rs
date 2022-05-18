fn main() {
    assert_eq!(
        Solution::can_partition_k_subsets(vec![4, 3, 2, 3, 5, 2, 1], 4),
        true
    );
    // assert_eq!(
    //     Solution::can_partition_k_subsets(vec![1, 2, 3, 4], 3),
    //     false
    // );
    // assert_eq!(
    //     Solution::can_partition_k_subsets(vec![2, 2, 2, 2, 3, 4, 5], 4),
    //     false
    // );
}

// https://www.geeksforgeeks.org/partition-set-k-subsets-equal-sum/
struct Solution {}
impl Solution {
    fn helper(
        nums: Vec<i32>,
        subset_sum: &mut Vec<i32>,
        taken: &mut Vec<bool>,
        subset: i32,
        k: i32,
        idx: usize,
        limit: i32,
    ) -> bool {
        if subset == subset_sum[idx] {
            if idx as i32 == k - 2 {
                return true;
            }
            return Solution::helper(
                nums.clone(),
                subset_sum,
                taken,
                subset,
                k,
                idx + 1,
                nums.len() as i32 - 1,
            );
        }

        if limit >= 0 {
            for i in (0..=limit as usize).rev() {
                if taken[i] {
                    continue;
                }
                if nums[i] + subset_sum[idx] <= subset {
                    taken[i] = true;
                    subset_sum[idx] += nums[i];
                    let nxt = Solution::helper(
                        nums.clone(),
                        subset_sum,
                        taken,
                        subset,
                        k,
                        idx,
                        i as i32 - 1,
                    );
                    taken[i] = false;
                    subset_sum[idx] -= nums[i];
                    if nxt {
                        return true;
                    }
                }
            }
        }
        return false;
    }
    pub fn can_partition_k_subsets(nums: Vec<i32>, k: i32) -> bool {
        if k == 1 {
            return true;
        }
        if nums.len() < k as usize {
            return false;
        }
        let sum: i32 = nums.iter().sum();
        if sum % k != 0 {
            return false;
        }
        let subset = sum / k;
        let mut subset_sum = vec![0; k as usize];
        let mut taken = vec![false; nums.len()];
        subset_sum[0] = nums.last().unwrap().clone();
        taken[nums.len() - 1] = true;
        return Solution::helper(
            nums.clone(),
            &mut subset_sum,
            &mut taken,
            subset,
            k,
            0,
            nums.len() as i32 - 1,
        );
    }
}
