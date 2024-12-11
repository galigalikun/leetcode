fn main() {
    assert_eq!(Solution::most_competitive(vec![3, 5, 2, 6], 2), vec![2, 6]);
    assert_eq!(
        Solution::most_competitive(vec![2, 4, 3, 3, 5, 4, 9, 6], 4),
        vec![2, 3, 3, 4]
    );
}

struct Solution;
impl Solution {
    pub fn most_competitive(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut stack = vec![];
        let k = k as usize;
        for (i, &num) in nums.iter().enumerate() {
            while !stack.is_empty()
                && stack.last().unwrap() > &num
                && stack.len() + nums.len() - i > k
            {
                stack.pop();
            }
            if stack.len() < k {
                stack.push(num);
            }
        }
        return stack;
    }
}
