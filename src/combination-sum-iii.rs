fn main() {
    assert_eq!(Solution::combination_sum3(3, 7), vec![vec![1, 2, 4]]);
    assert_eq!(
        Solution::combination_sum3(3, 9),
        vec![vec![1, 2, 6], vec![1, 3, 5], vec![2, 3, 4]]
    );
    // assert_eq!(Solution::combination_sum3(2, 18), vec![]);
    assert_eq!(
        Solution::combination_sum3(2, 6),
        vec![vec![1, 5], vec![2, 4]]
    );
    assert_eq!(
        Solution::combination_sum3(3, 15),
        vec![
            vec![1, 5, 9],
            vec![1, 6, 8],
            vec![2, 4, 9],
            vec![2, 5, 8],
            vec![2, 6, 7],
            vec![3, 4, 8],
            vec![3, 5, 7],
            vec![4, 5, 6]
        ]
    );
}

pub struct Solution {}
// https://akrad.hatenablog.com/entry/2020/09/13/213717
impl Solution {
    fn helper(
        k: usize,
        n: usize,
        start: usize,
        sum: usize,
        nums: &mut Vec<i32>,
        result: &mut Vec<Vec<i32>>,
    ) {
        if nums.len() > k || sum > n {
            return;
        }
        if nums.len() == k && sum == n {
            result.push(nums.to_vec());
        }

        for i in start..10 {
            nums.push(i as i32);
            Solution::helper(k, n, i + 1, sum + i, nums, result);

            nums.remove(nums.len() - 1);
        }
    }
    pub fn combination_sum3(k: i32, n: i32) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = vec![];
        Solution::helper(k as usize, n as usize, 1, 0, &mut vec![], &mut result);
        return result;
    }
}
