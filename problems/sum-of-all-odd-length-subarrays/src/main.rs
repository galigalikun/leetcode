fn main() {
    assert_eq!(Solution::sum_odd_length_subarrays(vec![1, 4, 2, 5, 3]), 58);
    assert_eq!(Solution::sum_odd_length_subarrays(vec![1, 2]), 3);
    assert_eq!(Solution::sum_odd_length_subarrays(vec![10, 11, 12]), 66);
}

struct Solution;
impl Solution {
    pub fn sum_odd_length_subarrays(arr: Vec<i32>) -> i32 {
        let mut sum = 0;
        for i in 0..arr.len() {
            let mut temp = 0;
            for j in i..arr.len() {
                temp += arr[j];
                if (j - i + 1) % 2 == 1 {
                    sum += temp;
                }
            }
        }
        sum
    }
}
