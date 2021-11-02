fn main() {
    assert_eq!(
        Solution::kth_smallest(vec![vec![1, 5, 9], vec![10, 11, 13], vec![12, 13, 15]], 8),
        13
    );

    assert_eq!(Solution::kth_smallest(vec![vec![-5]], 1), -5);
}

pub struct Solution {}
impl Solution {
    pub fn kth_smallest(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
        let mut v = vec![];
        for m in matrix {
            for i in m {
                v.push(i);
            }
        }
        v.sort();
        return v[k as usize - 1];
    }
}
