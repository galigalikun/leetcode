fn main() {
    assert_eq!(Solution::max_building(5, vec![vec![2, 1], vec![4, 1]]), 2);
    assert_eq!(Solution::max_building(6, vec![]), 5);
    assert_eq!(
        Solution::max_building(10, vec![vec![5, 3], vec![2, 5], vec![7, 4], vec![10, 3]]),
        5
    );
}

struct Solution;
impl Solution {
    pub fn max_building(n: i32, restrictions: Vec<Vec<i32>>) -> i32 {
        let mut restrictions = restrictions;
        restrictions.push(vec![1, 0]);
        restrictions.push(vec![n, n - 1]);
        restrictions.sort();
        let mut max_height = 0;
        for i in 1..restrictions.len() {
            let (x1, h1) = (restrictions[i - 1][0], restrictions[i - 1][1]);
            let (x2, h2) = (restrictions[i][0], restrictions[i][1]);
            max_height = max_height.max((h1 + h2 + x2 - x1) / 2);
            if x2 - x1 > h2 - h1 {
                max_height = max_height.max(h2 + (x2 - x1) / 2);
            }
        }
        return max_height;
    }
}
