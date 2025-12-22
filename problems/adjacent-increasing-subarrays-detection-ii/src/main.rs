fn main() {
    assert_eq!(Solution::max_increasing_subarrays(vec![2,5,7,8,9,2,3,4,3,1]), 3);
    assert_eq!(Solution::max_increasing_subarrays(vec![1,2,3,4,4,4,4,5,6,7]), 2);
}

struct Solution;
impl Solution {
    pub fn max_increasing_subarrays(nums: Vec<i32>) -> i32 {
        return nums
            .windows(2)
            .fold((1, 0), |(current_len, max_len), window| {
                if window[1] > window[0] {
                    let new_len = current_len + 1;
                    (new_len, max_len.max(new_len))
                } else {
                    (1, max_len)
                }
            })
            .1;
    }
}
