fn main() {
    assert_eq!(
        Solution::max_profit(2, vec![1, 2], vec![4, 3], vec![vec![1, 2]], 3),
        5
    );
    assert_eq!(
        Solution::max_profit(2, vec![3, 4], vec![5, 8], vec![vec![1, 2]], 4),
        4
    );
    assert_eq!(
        Solution::max_profit(
            3,
            vec![4, 6, 8],
            vec![7, 9, 11],
            vec![vec![1, 2], vec![1, 3]],
            10
        ),
        10
    );
}

struct Solution;
impl Solution {
    pub fn max_profit(
        n: i32,
        present: Vec<i32>,
        future: Vec<i32>,
        hierarchy: Vec<Vec<i32>>,
        budget: i32,
    ) -> i32 {
        // Placeholder implementation
        let mut max_profit = 0;
        for i in 0..n {
            let profit = future[i as usize] - present[i as usize];
            if profit > 0 {
                max_profit += profit;
            }
        }
        for relation in hierarchy {
            let v = (relation[1] - 1) as usize;
            if future[v] - present[v] > 0 {
                max_profit -= future[v] - present[v];
            }
        }
        max_profit.min(budget)
    }
}
