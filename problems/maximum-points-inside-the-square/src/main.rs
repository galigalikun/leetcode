fn main() {
    assert_eq!(
        Solution::max_points_inside_square(
            vec![
                vec![2, 2],
                vec![-1, -2],
                vec![-4, 4],
                vec![-3, 1],
                vec![3, -3]
            ],
            "abdca".to_string()
        ),
        2
    );
    assert_eq!(
        Solution::max_points_inside_square(
            vec![vec![1, 1], vec![-2, -2], vec![-2, 2]],
            "abb".to_string()
        ),
        1
    );
    assert_eq!(
        Solution::max_points_inside_square(
            vec![vec![1, 1], vec![-1, -1], vec![2, -2]],
            "ccd".to_string()
        ),
        0
    );
}

struct Solution;
impl Solution {
    pub fn max_points_inside_square(points: Vec<Vec<i32>>, s: String) -> i32 {
        let mut count = std::collections::HashMap::new();
        for (i, point) in points.iter().enumerate() {
            let ch = s.as_bytes()[i];
            let key = (
                point[0].signum(),
                point[1].signum(),
                (point[0].abs() as u32).leading_zeros(),
                (point[1].abs() as u32).leading_zeros(),
            );
            let entry = count.entry((key, ch)).or_insert(0);
            *entry += 1;
        }
        let mut max_count = 0;
        for ((_, _, _, _), ch) in count.keys() {
            let current_count: i32 = count
                .iter()
                .filter(|&((_, c), _)| c == ch)
                .map(|(_, &cnt)| cnt)
                .sum();
            if current_count > max_count {
                max_count = current_count;
            }
        }
        if max_count > 0 {
            return max_count;
        }
        return 0;
    }
}
