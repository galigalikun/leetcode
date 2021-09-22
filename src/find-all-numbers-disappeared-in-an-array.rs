fn main() {
    assert_eq!(
        Solution::find_disappeared_numbers(vec![4, 3, 2, 7, 8, 2, 3, 1]),
        vec![5, 6]
    );
    assert_eq!(Solution::find_disappeared_numbers(vec![1, 1]), vec![2]);
}

pub struct Solution {}
impl Solution {
    pub fn find_disappeared_numbers(nums: Vec<i32>) -> Vec<i32> {
        let mut result = (1..=nums.len() as i32)
            .map(|x| (x, true))
            .collect::<Vec<_>>();

        for n in nums {
            result[n as usize - 1] = (n - 1, false);
        }

        return result
            .into_iter()
            .filter(|(_, b)| *b)
            .map(|(n, _)| n)
            .collect::<Vec<_>>();
    }
}
