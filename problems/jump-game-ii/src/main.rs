fn main() {
    assert_eq!(Solution::jump(vec![2, 3, 1, 1, 4]), 2);
    assert_eq!(Solution::jump(vec![2, 3, 0, 1, 4]), 2);
}

struct Solution {}
// https://programmerstart.com/article/8618517245/
impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        let mut current = 0;
        let mut count = 0;
        while current < nums.len() - 1 {
            count += 1;
            if current + nums[current] as usize >= nums.len() - 1 {
                break;
            }
            let mut distance = 0;
            let mut index = current + 1;
            for i in current + 1..=current + nums[current] as usize {
                if nums[i] as usize + i > distance {
                    distance = nums[i] as usize + i;
                    index = i;
                }
            }
            current = index;
        }
        return count;
    }
}
