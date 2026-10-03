use std::collections::HashSet;

fn assert_close(a: f64, b: f64) {
    assert!((a - b).abs() < 1e-5, "{} != {}", a, b);
}

fn main() {
    assert_close(Solution::min_area_free_rect(vec![vec![1,2],vec![2,1],vec![1,0],vec![0,1]]), 2.0);
    assert_close(Solution::min_area_free_rect(vec![vec![0,1],vec![2,1],vec![1,1],vec![1,0],vec![2,0]]), 1.0);
    assert_close(Solution::min_area_free_rect(vec![vec![0,3],vec![1,2],vec![3,1],vec![1,3],vec![2,1]]), 0.0);
}

struct Solution;
impl Solution {
    pub fn min_area_free_rect(points: Vec<Vec<i32>>) -> f64 {
        if points.len() < 4 {
            return 0.0;
        }

        let mut set = HashSet::new();
        for p in &points {
            set.insert((p[0], p[1]));
        }

        if set.len() < 4 {
            return 0.0;
        }

        let points: Vec<(i32, i32)> = set.iter().copied().collect();
        let n = points.len();
        let mut min_area = f64::INFINITY;

        for i in 0..n {
            for j in 0..n {
                if i == j {
                    continue;
                }

                let (ax, ay) = points[i];
                let (bx, by) = points[j];
                let dx = bx - ax;
                let dy = by - ay;

                for k in 0..n {
                    if i == k || j == k {
                        continue;
                    }

                    let (cx, cy) = points[k];
                    let ex = cx - ax;
                    let ey = cy - ay;

                    if dx * ex + dy * ey != 0 {
                        continue;
                    }

                    let dx4 = bx + cx - ax;
                    let dy4 = by + cy - ay;
                    if !set.contains(&(dx4, dy4)) {
                        continue;
                    }

                    let side1 = ((dx * dx + dy * dy) as f64).sqrt();
                    let side2 = ((ex * ex + ey * ey) as f64).sqrt();
                    let area = side1 * side2;
                    if area > 0.0 && area < min_area {
                        min_area = area;
                    }
                }
            }
        }

        if min_area.is_infinite() {
            0.0
        } else {
            min_area
        }
    }
}
