fn main() {
    assert_eq!(
        Solution::group_the_people(vec![3, 3, 3, 3, 3, 1, 3]),
        vec![vec![5], vec![0, 1, 2], vec![3, 4, 6]]
    );
    assert_eq!(
        Solution::group_the_people(vec![2, 1, 3, 3, 3, 2]),
        vec![vec![1], vec![0, 5], vec![2, 3, 4]]
    );
}

struct Solution;
impl Solution {
    pub fn group_the_people(group_sizes: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = vec![];
        let mut map = std::collections::HashMap::new();
        for (i, size) in group_sizes.iter().enumerate() {
            let group = map.entry(size).or_insert(vec![]);
            group.push(i as i32);
            if group.len() == *size as usize {
                result.push(group.clone());
                map.remove(size);
            }
        }
        return result;
    }
}
