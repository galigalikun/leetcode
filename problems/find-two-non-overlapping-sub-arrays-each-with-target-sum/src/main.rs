fn main() {
    assert_eq!(Solution::min_sum_of_lengths(vec![3, 2, 2, 4, 3], 3), 2);
    assert_eq!(Solution::min_sum_of_lengths(vec![7, 3, 4, 7], 7), 2);
    assert_eq!(
        Solution::min_sum_of_lengths(vec![4, 3, 2, 6, 2, 3, 4], 6),
        -1
    );
}

struct Solution;
impl Solution {
    pub fn min_sum_of_lengths(arr: Vec<i32>, target: i32) -> i32 {
        let mut sum = 0;
        let mut left = 0;
        let mut right = 0;
        let mut min_len = i32::MAX;
        let mut min_len2 = i32::MAX;
        let mut res = i32::MAX;
        let mut map = std::collections::HashMap::new();
        map.insert(0, -1);
        while right < arr.len() {
            sum += arr[right];
            while sum > target {
                sum -= arr[left];
                left += 1;
            }
            if sum == target {
                let len = (right - left + 1) as i32;
                if len < min_len {
                    min_len2 = min_len;
                    min_len = len;
                } else if len < min_len2 {
                    min_len2 = len;
                }
                res = res.min(min_len + min_len2);
            }
            map.insert(sum, right as i32);
            right += 1;
        }
        return if res == i32::MAX { -1 } else { res };
    }
}
