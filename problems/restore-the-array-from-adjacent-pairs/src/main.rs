fn main() {
    assert_eq!(
        Solution::restore_array(vec![vec![2, 1], vec![3, 4], vec![3, 2]]),
        vec![1, 2, 3, 4]
    );
    assert_eq!(
        Solution::restore_array(vec![vec![4, -2], vec![1, 4], vec![-3, 1]]),
        vec![-2, 4, 1, -3]
    );
    assert_eq!(
        Solution::restore_array(vec![vec![100000, -100000]]),
        vec![100000, -100000]
    );
}

struct Solution;
impl Solution {
    pub fn restore_array(adjacent_pairs: Vec<Vec<i32>>) -> Vec<i32> {
        use std::collections::HashMap;
        let mut map = HashMap::new();
        for i in adjacent_pairs.clone() {
            map.entry(i[0]).or_insert(vec![]).push(i[1]);
            map.entry(i[1]).or_insert(vec![]).push(i[0]);
        }
        let mut ans = vec![];
        for (k, v) in map.iter() {
            if v.len() == 1 {
                ans.push(*k);
                ans.push(v[0]);
                break;
            }
        }
        while ans.len() < adjacent_pairs.len() + 1 {
            let last = *ans.last().unwrap();
            let next = map
                .get(&last)
                .unwrap()
                .iter()
                .find(|&&x| !ans.contains(&x))
                .unwrap();
            ans.push(*next);
        }
        ans
    }
}
