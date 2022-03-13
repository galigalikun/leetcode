fn main() {
    assert_eq!(
        Solution::combination_sum2(vec![10, 1, 2, 7, 6, 1, 5], 8),
        vec![vec![1, 1, 6], vec![1, 2, 5], vec![1, 7], vec![2, 6]]
    );
}

struct Solution {}
// https://www.programcreek.com/2014/04/leetcode-combination-sum-ii-java/
impl Solution {
    fn re_combination_sum(
        candidates: Vec<i32>,
        start: i32,
        target: i32,
        work: &mut Vec<i32>,
        result: &mut Vec<Vec<i32>>,
    ) {
        if target == 0 {
            result.push(work.to_vec());
            return;
        }
        if target < 0 {
            return;
        }

        let mut prev = -1;
        for j in start as usize..candidates.len() {
            if prev != candidates[j] {
                work.push(candidates[j]);
                Solution::re_combination_sum(
                    candidates.clone(),
                    (j + 1) as i32,
                    target - candidates[j],
                    work,
                    result,
                );
                work.remove(work.len() - 1);
                prev = candidates[j];
            }
        }
    }
    pub fn combination_sum2(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut s_candidates = candidates;
        s_candidates.sort();
        let mut result = Vec::new();
        let mut work = Vec::new();
        Solution::re_combination_sum(s_candidates, 0, target, &mut work, &mut result);

        return result;
    }
}
