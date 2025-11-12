fn main() {
    assert_eq!(
        Solution::min_rectangles_to_cover_points(
            vec![
                vec![2, 1],
                vec![1, 0],
                vec![1, 4],
                vec![1, 8],
                vec![3, 5],
                vec![4, 6]
            ],
            1
        ),
        2
    );
    assert_eq!(
        Solution::min_rectangles_to_cover_points(
            vec![
                vec![0, 0],
                vec![1, 1],
                vec![2, 2],
                vec![3, 3],
                vec![4, 4],
                vec![5, 5],
                vec![6, 6]
            ],
            2
        ),
        3
    );
    assert_eq!(
        Solution::min_rectangles_to_cover_points(vec![vec![2, 3], vec![1, 2]], 2),
        1
    );
}

struct Solution;
impl Solution {
    pub fn min_rectangles_to_cover_points(points: Vec<Vec<i32>>, w: i32) -> i32 {
        let mut points = points;
        points.sort_unstable_by_key(|p| p[0]);
        let mut count = 0;
        let mut i = 0;
        while i < points.len() {
            count += 1;
            let mut j = i + 1;
            while j < points.len() && points[j][0] <= points[i][0] + w {
                j += 1;
            }
            i = j;
        }
        return count;
    }
}
