fn main() {
    assert_eq!(Solution::minimum_incompatibility(vec![1, 2, 1, 4], 2), 4);
    assert_eq!(
        Solution::minimum_incompatibility(vec![6, 3, 8, 1, 3, 1, 2, 2], 4),
        6
    );
    assert_eq!(
        Solution::minimum_incompatibility(vec![5, 3, 3, 6, 3, 3], 3),
        -1
    );
}

struct Solution;
impl Solution {
    pub fn minimum_incompatibility(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        let k = k as usize;
        let mut dp = vec![std::i32::MAX; 1 << n];
        let mut cnt = vec![0; n];
        let mut ans = std::i32::MAX;
        let mut group = vec![0; n];
        let mut group_size = vec![0; k];
        for i in 0..n {
            cnt[nums[i] as usize] += 1;
        }
        for i in 0..1 << n {
            let mut ok = true;
            for j in 0..n {
                if i & 1 << j == 0 {
                    continue;
                }
                if cnt[nums[j] as usize] > 1 {
                    cnt[nums[j] as usize] -= 1;
                } else {
                    ok = false;
                    break;
                }
            }
            if ok {
                dp[i] = 0;
            }
        }
        for i in 0..1 << n {
            if dp[i] == 0 {
                for j in 0..n {
                    if i & 1 << j == 0 {
                        let next = i | 1 << j;
                        dp[next] = dp[next].min(dp[i] + 1);
                    }
                }
            }
        }
        for i in 0..1 << n {
            if dp[i] == 0 && i.count_ones() == n as u32 / k as u32 {
                let mut ok = true;
                for j in 0..k {
                    group_size[j] = 0;
                }
                for j in 0..n {
                    if i & 1 << j == 0 {
                        continue;
                    }
                    group[j] = -1;
                    for l in 0..k {
                        if group_size[l] == 0 {
                            group[j] = l as i32;
                            group_size[l] += 1;
                            break;
                        }
                    }
                    if group[j] == -1 {
                        ok = false;
                        break;
                    }
                }
                if ok {
                    let mut sum = 0;
                    for j in 0..k {
                        let mut min = std::i32::MAX;
                        let mut max = std::i32::MIN;
                        for l in 0..n {
                            if group[l] == j as i32 {
                                min = min.min(nums[l]);
                                max = max.max(nums[l]);
                            }
                        }
                        sum += max - min;
                    }
                    ans = ans.min(sum);
                }
            }
        }
        if ans == std::i32::MAX {
            return -1;
        }
        return ans;
    }
}
