fn main() {
    assert_eq!(
        Solution::largest_triangle_area(vec![
            vec![0, 0],
            vec![0, 1],
            vec![1, 0],
            vec![0, 2],
            vec![2, 0]
        ]),
        2.0
    );
    assert_eq!(
        Solution::largest_triangle_area(vec![vec![1, 0], vec![0, 0], vec![0, 1]]),
        0.5
    );
    assert_eq!(
        Solution::largest_triangle_area(vec![vec![4, 6], vec![6, 5], vec![3, 1]]),
        5.5
    );
}

struct Solution {}
impl Solution {
    pub fn largest_triangle_area(points: Vec<Vec<i32>>) -> f64 {
        let n = points.len();
        let mut max_area = 0.0_f64;

        for i in 0..n {
            for j in (i + 1)..n {
                for k in (j + 1)..n {
                    let ax = points[i][0] as f64;
                    let ay = points[i][1] as f64;
                    let bx = points[j][0] as f64;
                    let by = points[j][1] as f64;
                    let cx = points[k][0] as f64;
                    let cy = points[k][1] as f64;

                    let twice_area = ((bx - ax) * (cy - ay) - (by - ay) * (cx - ax)).abs();
                    let area = twice_area * 0.5;
                    if area > max_area {
                        max_area = area;
                    }
                }
            }
        }

        max_area
    }
}
