fn main() {
    assert_eq!(
        Solution::count_points(
            vec![vec![1, 3], vec![3, 3], vec![5, 3], vec![2, 2]],
            vec![vec![2, 3, 1], vec![4, 3, 1], vec![1, 1, 2]]
        ),
        vec![3, 2, 2]
    );
    assert_eq!(
        Solution::count_points(
            vec![vec![1, 1], vec![2, 2], vec![3, 3], vec![4, 4], vec![5, 5]],
            vec![vec![1, 2, 2], vec![2, 2, 2], vec![4, 3, 2], vec![4, 3, 3]]
        ),
        vec![2, 3, 2, 4]
    );
}

struct Solution;
impl Solution {
    pub fn count_points(points: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        // Create a vector to store the results
        let mut results = vec![0; queries.len()];
        // Iterate over each query
        for (i, query) in queries.iter().enumerate() {
            // Extract the center and radius of the query
            let (x, y, r) = (query[0], query[1], query[2]);
            // Count the number of points within the radius
            results[i] = points
                .iter()
                .filter(|point| {
                    let dx = point[0] - x;
                    let dy = point[1] - y;
                    dx * dx + dy * dy <= r * r
                })
                .count() as i32;
        }
        // Return the results
        results
    }
}
