fn main() {
    assert_eq!(Solution::min_k_bit_flips(vec![0, 1, 0], 1), 2);
    assert_eq!(Solution::min_k_bit_flips(vec![1, 1, 0], 2), -1);
    assert_eq!(
        Solution::min_k_bit_flips(vec![0, 0, 0, 1, 0, 1, 1, 0], 3),
        3
    );
    assert_eq!(Solution::min_k_bit_flips(vec![0, 0, 1, 0, 0], 2), 2);
    assert_eq!(Solution::min_k_bit_flips(vec![1, 1, 1], 2), 0);
}

struct Solution;
impl Solution {
    pub fn min_k_bit_flips(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        let mut diff = vec![0; n + 1];
        let mut active = 0;
        let mut flips = 0;

        for i in 0..n {
            active += diff[i];
            let effective = nums[i] ^ (active & 1);

            if effective == 0 {
                let end = i + k as usize;
                if end > n {
                    return -1;
                }

                flips += 1;
                active += 1;
                diff[end] -= 1;
            }
        }

        flips
    }
}
