fn main() {
    let mut nums = vec![1, 2, 3, 4, 5, 6, 7];
    Solution::rotate(&mut nums, 3);
    assert_eq!(nums, vec![5, 6, 7, 1, 2, 3, 4]);

    nums = vec![-1, -100, 3, 99];
    Solution::rotate(&mut nums, 2);
    assert_eq!(nums, vec![3, 99, -1, -100]);

    nums = vec![-1];
    Solution::rotate(&mut nums, 2);
    assert_eq!(nums, vec![-1]);
}

pub struct Solution {}
impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        let rotate = k as usize % nums.len();
        let mut i = 0;
        for num in [
            &nums[nums.len() - rotate..nums.len()],
            &nums[0..nums.len() - rotate],
        ]
        .concat()
        {
            nums[i] = num;
            i += 1;
        }
    }
}
