fn main() {
    assert_eq!(
        Solution::get_min_dist_sum(vec![vec![0, 1], vec![1, 0], vec![1, 2], vec![2, 1]]),
        4.0
    );
    assert_eq!(
        Solution::get_min_dist_sum(vec![vec![1, 1], vec![3, 3]]),
        2.82843
    );
}

struct Solution;
impl Solution {
    fn get_dist(positions: &Vec<Vec<i32>>, x: f64, y: f64) -> f64 {
        let mut dist = 0.0;
        for pos in positions {
            dist += ((pos[0] as f64 - x) * (pos[0] as f64 - x)
                + (pos[1] as f64 - y) * (pos[1] as f64 - y))
                .sqrt();
        }
        return dist;
    }
    pub fn get_min_dist_sum(positions: Vec<Vec<i32>>) -> f64 {
        let mut x = 0.0;
        let mut y = 0.0;
        for pos in &positions {
            x += pos[0] as f64;
            y += pos[1] as f64;
        }
        x /= positions.len() as f64;
        y /= positions.len() as f64;
        let mut step = 100.0;
        let mut ans = Self::get_dist(&positions, x, y);
        while step > 1e-6 {
            let mut found = false;
            for i in -1..=1 {
                for j in -1..=1 {
                    let nx = x + step * i as f64;
                    let ny = y + step * j as f64;
                    let dist = Self::get_dist(&positions, nx, ny);
                    if dist < ans {
                        ans = dist;
                        x = nx;
                        y = ny;
                        found = true;
                    }
                }
            }
            if !found {
                step /= 2.0;
            }
        }
        return ans;
    }
}
