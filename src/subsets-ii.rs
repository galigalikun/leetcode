fn main() {
    assert_eq!(
        Solution::subsets_with_dup(vec![1, 2, 2]),
        vec![
            vec![],
            vec![1],
            vec![1, 2],
            vec![1, 2, 2],
            vec![2],
            vec![2, 2]
        ]
    );
}

pub struct Solution {}
impl Solution {
    fn dfs(nums: Vec<i32>, result: &mut Vec<Vec<i32>>, prev: &mut Vec<i32>, idx: usize) {
        if !result.contains(prev) {
            result.push((&prev).to_vec());
        }

        for i in idx..nums.len() {
            prev.push(nums[i]);
            Solution::dfs(nums.clone(), result, prev, i+1);
            prev.remove(prev.len()-1);
        }
    }
    pub fn subsets_with_dup(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        let mut prev = Vec::new();
        let mut nn = nums;
        nn.sort();
        Solution::dfs(nn, &mut result, &mut prev, 0);
        return result;
    }
}
