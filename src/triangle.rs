fn main() {
    assert_eq!(
        Solution::minimum_total(vec![vec![2], vec![3, 4], vec![6, 5, 7], vec![4, 1, 8, 3]]),
        11
    );
    assert_eq!(
        Solution::minimum_total(vec![vec![-1], vec![2, 3], vec![1, -1, -3]]),
        -1
    );
}

pub struct Solution {}
// https://www.programcreek.com/2013/01/leetcode-triangle-java/
impl Solution {
    pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
        let mut total: Vec<i32> = Vec::with_capacity(triangle.len());
        for last in triangle.last() {
            for &num in last {
                total.push(num);
            }
        }

        for i in (0..triangle.len() - 1).rev() {
            for j in 0..triangle[i].len() {
                total[j] = triangle[i][j] + std::cmp::min(total[j], total[j + 1]);
            }
        }

        return total[0];
    }
}
