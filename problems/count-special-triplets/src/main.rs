fn main() {
    assert_eq!(Solution::special_triplets(vec![6,3,6]), 1);
    assert_eq!(Solution::special_triplets(vec![0,1,0,0]), 1);
    assert_eq!(Solution::special_triplets(vec![8,4,2,8,4]), 2);
}

struct Solution;
impl Solution {
    pub fn special_triplets(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut count = 0;

        for j in 1..n-1 {
            let mut left_smaller = 0;
            let mut right_greater = 0;

            for i in 0..j {
                if nums[i] < nums[j] {
                    left_smaller += 1;
                }
            }

            for k in j+1..n {
                if nums[k] > nums[j] {
                    right_greater += 1;
                }
            }

            count += left_smaller * right_greater;
        }

        count
    }
}
