use std::vec;

fn main() {
    assert_eq!(
        Solution::process_queries(vec![3, 1, 2, 1], 5),
        vec![2, 1, 2, 1]
    );
    assert_eq!(
        Solution::process_queries(vec![4, 1, 2, 2], 4),
        vec![3, 1, 2, 0]
    );
    assert_eq!(
        Solution::process_queries(vec![7, 5, 5, 8, 3], 8),
        vec![6, 5, 0, 7, 5]
    );
}

struct Solution;
impl Solution {
    pub fn process_queries(queries: Vec<i32>, m: i32) -> Vec<i32> {
        let mut p = vec![];
        for i in 1..=m {
            p.push(i);
        }
        let mut res = vec![];
        for q in queries {
            let idx = p.iter().position(|&x| x == q).unwrap() as i32;
            res.push(idx);
            p.remove(idx as usize);
            p.insert(0, q);
        }
        return res;
    }
}
