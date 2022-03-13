fn main() {
    assert_eq!(
        Solution::largest_divisible_subset(vec![1, 2, 3]),
        vec![1, 2]
    );

    assert_eq!(
        Solution::largest_divisible_subset(vec![1, 2, 4, 8]),
        vec![1, 2, 4, 8]
    );

    assert_eq!(
        Solution::largest_divisible_subset(vec![2, 3, 4, 8]),
        vec![2, 4, 8]
    );

    assert_eq!(
        Solution::largest_divisible_subset(vec![4, 8, 10, 240]),
        vec![4, 8, 240]
    );

    assert_eq!(
        Solution::largest_divisible_subset(vec![3, 4, 16, 8]),
        vec![4, 8, 16]
    );
}

struct Solution {}
// https://www.geeksforgeeks.org/largest-divisible-subset-array/
impl Solution {
    pub fn largest_divisible_subset(nums: Vec<i32>) -> Vec<i32> {
        let mut div = vec![1; nums.len()];
        let mut prev: Vec<i32> = vec![-1; nums.len()];

        let mut max_ind: i32 = 0;
        let mut s_nums = nums;
        s_nums.sort();
        for i in 1..s_nums.len() {
            for j in 0..i {
                if s_nums[i] % s_nums[j] == 0 && div[i] < div[j] + 1 {
                    prev[i] = j as i32;
                    div[i] = div[j] + 1;
                }
            }

            if div[i] > div[max_ind as usize] {
                max_ind = i as i32;
            }
        }
        let mut result = vec![];
        while max_ind >= 0 {
            result.push(s_nums[max_ind as usize]);
            max_ind = prev[max_ind as usize];
        }

        return result;
    }
}
