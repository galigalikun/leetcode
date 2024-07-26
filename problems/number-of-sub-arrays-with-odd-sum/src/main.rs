fn main() {
    assert_eq!(Solution::num_of_subarrays(vec![1, 3, 5]), 4);
    assert_eq!(Solution::num_of_subarrays(vec![2, 4, 6]), 0);
    assert_eq!(Solution::num_of_subarrays(vec![1, 2, 3, 4, 5, 6, 7]), 16);
}

struct Solution;
impl Solution {
    pub fn num_of_subarrays(arr: Vec<i32>) -> i32 {
        let mut odd_count = 0;
        let mut even_count = 1;
        let mut sum = 0;
        for num in arr {
            sum += num;
            if sum % 2 == 0 {
                even_count += 1;
            } else {
                odd_count += 1;
            }
        }
        if odd_count == 0 {
            0
        } else {
            odd_count * even_count
        }
    }
}
