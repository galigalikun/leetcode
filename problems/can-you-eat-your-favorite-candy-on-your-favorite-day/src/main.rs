fn main() {
    assert_eq!(
        Solution::can_eat(
            vec![7, 4, 5, 3, 8],
            vec![vec![0, 2, 2], vec![4, 2, 4], vec![2, 13, 1000000000]]
        ),
        vec![true, false, true]
    );
    assert_eq!(
        Solution::can_eat(
            vec![5, 2, 6, 4, 1],
            vec![vec![3, 1, 2], vec![4, 10, 3], vec![3, 10, 100]]
        ),
        vec![false, true, true]
    );
}

struct Solution;
impl Solution {
    pub fn can_eat(candies_count: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<bool> {
        let mut prefix_sum = vec![0];
        for i in 0..candies_count.len() {
            prefix_sum.push(prefix_sum[i] + candies_count[i] as i128);
        }
        let mut res = vec![];
        for query in queries {
            let (favorite_type, favorite_day, daily_cap) =
                (query[0] as usize, query[1] as i128, query[2] as i128);
            let min_eat = favorite_day + 1;
            let max_eat = (favorite_day + 1) * daily_cap;
            let min_candies = prefix_sum[favorite_type] + 1;
            let max_candies = prefix_sum[favorite_type + 1];
            res.push(min_candies <= max_eat && max_candies >= min_eat);
        }
        res
    }
}
