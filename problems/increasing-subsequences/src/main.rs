fn main() {
    assert_eq!(
        Solution::find_subsequences(vec![4, 6, 7, 7]),
        vec![
            vec![4, 6],
            vec![4, 6, 7],
            vec![4, 6, 7, 7],
            vec![4, 7],
            vec![4, 7, 7],
            vec![6, 7],
            vec![6, 7, 7],
            vec![7, 7]
        ]
    );
    assert_eq!(
        Solution::find_subsequences(vec![4, 4, 3, 2, 1]),
        vec![vec![4, 4]]
    );
}

struct Solution {}
// https://ttzztt.gitbooks.io/lc/content/increasing-subsequences.html
use std::collections::HashSet;
impl Solution {
    fn dfs(res: &mut Vec<Vec<i32>>, ele: &mut Vec<i32>, pos: usize, nums: Vec<i32>) {
        if ele.len() >= 2 {
            res.push(ele.to_vec());
        }
        let mut s = HashSet::with_capacity(nums.len());
        for i in pos..nums.len() {
            if (ele.len() == 0 || ele.get(ele.len() - 1) <= Some(&nums[i])) && !s.contains(&nums[i])
            {
                ele.push(nums[i]);
                Solution::dfs(res, ele, i + 1, nums.clone());
                ele.remove(ele.len() - 1);
                s.insert(nums[i]);
            }
        }
    }
    pub fn find_subsequences(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res = vec![];
        let mut ele = vec![];
        Solution::dfs(&mut res, &mut ele, 0, nums);
        return res;
    }
}
