fn main() {
    assert_eq!(Solution::count_trapezoids(vec![[-3,2],[3,0],[2,3],[3,2],[2,-3]].iter().map(|v| v.to_vec()).collect()), 2);
    assert_eq!(Solution::count_trapezoids(vec![[0,0],[1,0],[0,1],[2,1]].iter().map(|v| v.to_vec()).collect()), 1);
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn from_vec(v: Vec<i32>) -> Self {
        Self {
            x: v[0] as i64,
            y: v[1] as i64,
        }
    }
}

struct Solution;
impl Solution {
    pub fn count_trapezoids(points: Vec<Vec<i32>>) -> i32 {
        let pts: Vec<Point> = points.into_iter().map(Point::from_vec).collect();
        let n = pts.len();
        if n < 4 {
            return 0;
        }

        let mut total = 0i64;
        for a in 0..n - 3 {
            for b in a + 1..n - 2 {
                for c in b + 1..n - 1 {
                    for d in c + 1..n {
                        let subset = [pts[a], pts[b], pts[c], pts[d]];
                        if is_trapezoid(&subset) {
                            total += 1;
                        }
                    }
                }
            }
        }

        total as i32
    }
}

fn is_trapezoid(points: &[Point; 4]) -> bool {
    let hull = convex_hull(points);
    if hull.len() != 4 {
        return false;
    }

    is_parallel(hull[0], hull[1], hull[2], hull[3])
        || is_parallel(hull[1], hull[2], hull[3], hull[0])
}

fn convex_hull(points: &[Point]) -> Vec<Point> {
    let mut pts = points.to_vec();
    pts.sort_by(|a, b| {
        if a.x == b.x {
            a.y.cmp(&b.y)
        } else {
            a.x.cmp(&b.x)
        }
    });
    pts.dedup();

    if pts.len() <= 2 {
        return pts;
    }

    // Monotonic chain hull works well for the tiny point sets involved here.
    let mut lower: Vec<Point> = Vec::new();
    for &p in &pts {
        while lower.len() >= 2
            && cross(lower[lower.len() - 2], lower[lower.len() - 1], p) <= 0
        {
            lower.pop();
        }
        lower.push(p);
    }

    let mut upper: Vec<Point> = Vec::new();
    for &p in pts.iter().rev() {
        while upper.len() >= 2
            && cross(upper[upper.len() - 2], upper[upper.len() - 1], p) <= 0
        {
            upper.pop();
        }
        upper.push(p);
    }

    lower.pop();
    upper.pop();
    lower.extend(upper);
    lower
}

fn cross(o: Point, a: Point, b: Point) -> i128 {
    let ax = (a.x - o.x) as i128;
    let ay = (a.y - o.y) as i128;
    let bx = (b.x - o.x) as i128;
    let by = (b.y - o.y) as i128;
    ax * by - ay * bx
}

fn is_parallel(a: Point, b: Point, c: Point, d: Point) -> bool {
    let dx1 = (b.x - a.x) as i128;
    let dy1 = (b.y - a.y) as i128;
    let dx2 = (d.x - c.x) as i128;
    let dy2 = (d.y - c.y) as i128;
    dy1 * dx2 == dy2 * dx1
}
