fn main() {
    assert_eq!(
        Solution::find_diagonal_order(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]),
        vec![1, 4, 2, 7, 5, 3, 8, 6, 9]
    );
    assert_eq!(
        Solution::find_diagonal_order(vec![
            vec![1, 2, 3, 4, 5],
            vec![6, 7],
            vec![8],
            vec![9, 10, 11],
            vec![12, 13, 14, 15, 16]
        ]),
        vec![1, 6, 2, 8, 7, 3, 9, 4, 12, 10, 5, 13, 11, 14, 15, 16]
    );
}

struct Solution;
impl Solution {
    pub fn find_diagonal_order(nums: Vec<Vec<i32>>) -> Vec<i32> {
        let mut res = vec![];
        let mut map = std::collections::HashMap::new();
        for i in 0..nums.len() {
            for j in 0..nums[i].len() {
                map.entry(i + j).or_insert(vec![]).push(nums[i][j]);
            }
        }
        for i in 0..map.len() {
            if i % 2 == 0 {
                res.extend(map[&i].iter().rev());
            } else {
                res.extend(map[&i].iter());
            }
        }
        return res;
    }
}
