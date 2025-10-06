fn main() {
    assert_eq!(
        Solution::smallest_missing_value_subtree(vec![-1, 0, 0, 2], vec![1, 2, 3, 4]),
        vec![5, 1, 1, 1]
    );
    assert_eq!(
        Solution::smallest_missing_value_subtree(vec![-1, 0, 1, 0, 3, 3], vec![5, 4, 6, 2, 1, 3]),
        vec![7, 1, 1, 4, 2, 1]
    );
    assert_eq!(
        Solution::smallest_missing_value_subtree(
            vec![-1, 2, 3, 0, 2, 4, 1],
            vec![2, 3, 4, 5, 6, 7, 8]
        ),
        vec![1, 1, 1, 1, 1, 1, 1]
    );
}

struct Solution;
impl Solution {
    fn dfs(
        node: usize,
        tree: &Vec<Vec<usize>>,
        nums: &Vec<i32>,
        result: &mut Vec<i32>,
        missing: &mut i32,
    ) {
        if nums[node] == *missing {
            *missing += 1;
        }
        for &child in &tree[node] {
            Self::dfs(child, tree, nums, result, missing);
        }
        result[node] = *missing;
    }
    pub fn smallest_missing_value_subtree(parents: Vec<i32>, nums: Vec<i32>) -> Vec<i32> {
        let n = parents.len();
        let mut tree = vec![vec![]; n];
        let mut result = vec![1; n];
        let mut num_to_index = vec![None; n as usize + 2];
        for (i, &num) in nums.iter().enumerate() {
            num_to_index[num as usize] = Some(i);
        }
        for (i, &p) in parents.iter().enumerate() {
            if p != -1 {
                tree[p as usize].push(i);
            }
        }
        if num_to_index[1].is_none() {
            return result;
        }
        let mut current = num_to_index[1].unwrap();
        let mut missing = 1;
        while current != usize::MAX {
            Self::dfs(current, &tree, &nums, &mut result, &mut missing);
            current = if parents[current] == -1 {
                usize::MAX
            } else {
                parents[current] as usize
            };
        }
        result
    }
}
