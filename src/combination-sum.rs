fn main() {
    // assert_eq!(
    //     Solution::combination_sum(vec![2, 3, 6, 7], 7),
    //     vec![vec![2, 2, 3], vec![7]]
    // );
    assert_eq!(
        Solution::combination_sum(vec![2, 3, 5], 8),
        vec![vec![2, 2, 2, 2], vec![2, 3, 3], vec![3, 5]]
    );
}

pub struct Solution {}
// https://www.programcreek.com/2014/02/leetcode-combination-sum-java/
impl Solution {
    fn re_combination_sum(
        candidates: Vec<i32>,
        start: usize,
        target: i32,
        sum: i32,
        work: &mut Vec<i32>,
        result: &mut Vec<Vec<i32>>,
    ) {
        if sum > target {
            return;
        }

        if sum == target {
            result.push(work.to_vec());
        }

        for i in start..candidates.len() {
            work.push(candidates[i]);
            Solution::re_combination_sum(candidates.clone(), i, target, sum + candidates[i], work, result);
            work.remove(work.len() - 1);
        }
    }
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        let mut work = Vec::new();
        Solution::re_combination_sum(candidates, 0, target, 0, &mut work, &mut result);

        return result;
    }
}
