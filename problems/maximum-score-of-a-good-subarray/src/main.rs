fn main() {
    assert_eq!(Solution::maximum_score(vec![1, 2, 3, 4, 5], 5), 12);
    assert_eq!(Solution::maximum_score(vec![5, 5, 4, 5, 4, 1, 1, 1], 0), 20);
}

struct Solution;
impl Solution {
    pub fn maximum_score(nums: Vec<i32>, k: i32) -> i32 {
        let mut nums = nums;
        nums.sort_unstable();
        let n = nums.len();
        let mut ans = 0;
        for i in 0..n {
            if nums[i] > k {
                break;
            }
            for j in (i + 1)..n {
                if nums[j] > k {
                    break;
                }
                for l in (j + 1)..n {
                    if nums[l] > k {
                        break;
                    }
                    ans = ans.max(nums[i] + nums[j] + nums[l]);
                }
            }
        }
        ans
    }
}
