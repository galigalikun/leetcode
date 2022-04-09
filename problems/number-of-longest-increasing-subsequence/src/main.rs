fn main() {
    assert_eq!(Solution::find_number_of_lis(vec![1, 3, 5, 4, 7]), 2);
    assert_eq!(Solution::find_number_of_lis(vec![2, 2, 2, 2, 2]), 5);
}

// https://www.geeksforgeeks.org/number-of-longest-increasing-subsequences/
struct Solution {}
impl Solution {
    pub fn find_number_of_lis(nums: Vec<i32>) -> i32 {
        if nums.len() == 0 {
            return 0;
        }

        let mut dp_l = vec![1; nums.len()];
        let mut dp_c = vec![1; nums.len()];
        for i in 0..nums.len() {
            for j in 0..i {
                if nums[i] <= nums[j] {
                    continue;
                }
                if dp_l[j] + 1 > dp_l[i] {
                    dp_l[i] = dp_l[j] + 1;
                    dp_c[i] = dp_c[j];
                } else if dp_l[j] + 1 == dp_l[i] {
                    dp_c[i] += dp_c[j];
                }
            }
        }

        let max_length = dp_l.clone().into_iter().max().unwrap();

        let mut count = 0;
        for i in 0..nums.len() {
            if dp_l[i] == max_length {
                count += dp_c[i];
            }
        }
        return count;
    }
}
