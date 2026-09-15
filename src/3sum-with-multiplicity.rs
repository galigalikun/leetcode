fn main() {
    assert_eq!(
        20,
        Solution::three_sum_multi(vec![1, 1, 2, 2, 3, 3, 4, 4, 5, 5], 8)
    );
    assert_eq!(12, Solution::three_sum_multi(vec![1, 1, 2, 2, 2, 2], 5));
}
struct Solution;
impl Solution {
    pub fn three_sum_multi(arr: Vec<i32>, target: i32) -> i32 {
        const MOD: i64 = 1_000_000_007;

        let mut nums = arr;
        nums.sort_unstable();

        let n = nums.len();
        let mut answer: i64 = 0;

        for i in 0..n {
            let mut left = i + 1;
            if left >= n {
                break;
            }
            let mut right = n - 1;

            while left < right {
                let sum = nums[i] + nums[left] + nums[right];
                if sum < target {
                    left += 1;
                } else if sum > target {
                    right -= 1;
                } else if nums[left] != nums[right] {
                    let left_value = nums[left];
                    let right_value = nums[right];
                    let mut left_count: i64 = 0;
                    let mut right_count: i64 = 0;

                    while left < right && nums[left] == left_value {
                        left_count += 1;
                        left += 1;
                    }
                    while left <= right && nums[right] == right_value {
                        right_count += 1;
                        if right == 0 {
                            break;
                        }
                        right -= 1;
                    }

                    answer = (answer + (left_count * right_count) % MOD) % MOD;
                } else {
                    let count = (right - left + 1) as i64;
                    answer = (answer + (count * (count - 1) / 2) % MOD) % MOD;
                    break;
                }
            }
        }

        answer as i32
    }
}
