fn main() {
    assert_eq!(
        Solution::combine(4, 2),
        vec![[1, 2], [1, 3], [1, 4], [2, 3], [2, 4], [3, 4],]
    );
    assert_eq!(Solution::combine(1, 1), vec![[1]]);
    assert_eq!(Solution::combine(2, 1), vec![[1], [2]]);
    assert_eq!(Solution::combine(3, 3), vec![[1, 2, 3]]);
    assert_eq!(
        Solution::combine(4, 3),
        vec![[1, 2, 3], [1, 2, 4], [1, 3, 4], [2, 3, 4]]
    );
}

pub struct Solution {}
// https://www.youtube.com/watch?v=iLuYoPwAhmM
impl Solution {
    fn def(subset: &mut Vec<Vec<i32>>, index: i32, current: &mut Vec<i32>, n: i32, k: i32) {
        if current.len() == k as usize {
            subset.push(current.to_vec());
            return;
        }

        for i in index..=n {
            current.push(i);
            Solution::def(subset, i+1, current, n, k);
            current.remove(current.len()-1);
        }
    }
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        let mut subset = Vec::new();
        Solution::def(&mut subset, 1, &mut vec![], n, k);

        return subset;
    }
}
