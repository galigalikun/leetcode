fn main() {
    assert_eq!(
        Solution::min_height_shelves(
            vec![
                vec![1, 1],
                vec![2, 3],
                vec![2, 3],
                vec![1, 1],
                vec![1, 1],
                vec![1, 1],
                vec![1, 2]
            ],
            4
        ),
        6
    );
    assert_eq!(
        Solution::min_height_shelves(vec![vec![1, 3], vec![2, 4], vec![3, 2]], 6),
        4
    );
}

struct Solution;
impl Solution {
    pub fn min_height_shelves(books: Vec<Vec<i32>>, shelf_width: i32) -> i32 {
        let mut dp = vec![0; books.len() + 1];
        for i in 1..=books.len() {
            let mut width = books[i - 1][0];
            let mut height = books[i - 1][1];
            dp[i] = dp[i - 1] + height;
            for j in (1..i).rev() {
                width += books[j - 1][0];
                if width > shelf_width {
                    break;
                }
                height = height.max(books[j - 1][1]);
                dp[i] = dp[i].min(dp[j - 1] + height);
            }
        }
        return dp[books.len()];
    }
}
