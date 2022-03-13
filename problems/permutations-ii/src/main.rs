fn main() {
    assert_eq!(
        Solution::permute_unique(vec![1, 1, 2]),
        vec![[1, 1, 2], [1, 2, 1], [2, 1, 1]]
    );
    assert_eq!(
        Solution::permute_unique(vec![1, 2, 3]),
        vec![
            [1, 2, 3],
            [1, 3, 2],
            [2, 1, 3],
            [2, 3, 1],
            [3, 1, 2],
            [3, 2, 1]
        ]
    );
}

struct Solution {}
// https://stlisacity.hatenablog.com/entry/2018/06/20/232830
impl Solution {
    fn helper(
        nums: Vec<i32>,
        result: &mut Vec<Vec<i32>>,
        list: &mut Vec<i32>,
        visited: &mut Vec<bool>,
    ) {
        if nums.len() == list.len() {
            result.push(list.to_vec());
            return;
        }

        for i in 0..nums.len() {
            if i != 0 && nums[i] == nums[i - 1] && visited[i - 1] {
                continue;
            }
            if !visited[i] {
                visited[i] = true;
                list.push(nums[i]);
                Solution::helper(nums.clone(), result, list, visited);
                list.remove(list.len() - 1);
                visited[i] = false;
            }
        }
    }
    pub fn permute_unique(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = vec![];
        let mut list = nums;
        list.sort();
        let mut visited = vec![false; list.len()];
        Solution::helper(list, &mut result, &mut vec![], &mut visited);
        return result;
    }
}
