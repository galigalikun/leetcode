fn main() {
    assert_eq!(
        Solution::num_points(vec![vec![-2, 0], vec![2, 0], vec![0, 2], vec![0, -2]], 2),
        4
    );
    assert_eq!(
        Solution::num_points(
            vec![
                vec![-3, 0],
                vec![3, 0],
                vec![2, 6],
                vec![5, 4],
                vec![0, 9],
                vec![7, 8]
            ],
            5
        ),
        5
    );
}

struct Solution;
impl Solution {
    pub fn num_points(darts: Vec<Vec<i32>>, r: i32) -> i32 {
        let mut res = 1;
        for i in 0..darts.len() {
            for j in 0..darts.len() {
                if i == j {
                    continue;
                }
                let (x1, y1) = (darts[i][0], darts[i][1]);
                let (x2, y2) = (darts[j][0], darts[j][1]);
                let d = ((x1 - x2).pow(2) + (y1 - y2).pow(2)) as f64;
                if d > (r * 2) as f64 {
                    continue;
                }
                let x0 = (x1 + x2) as f64 / 2.0;
                let y0 = (y1 + y2) as f64 / 2.0;
                let l = (r.pow(2) as f64 - d / 4.0).sqrt();
                let x3 = x0 + l * (y1 - y2) as f64 / (d as f64).sqrt();
                let y3 = y0 + l * (x2 - x1) as f64 / (d as f64).sqrt();
                let x4 = x0 - l * (y1 - y2) as f64 / (d as f64).sqrt();
                let y4 = y0 - l * (x2 - x1) as f64 / (d as f64).sqrt();
                let mut cnt = 0;
                for k in 0..darts.len() {
                    if ((darts[k][0] as f64 - x3).powi(2) + (darts[k][1] as f64 - y3).powi(2)
                        - (r as f64).powi(2))
                    .abs()
                        < 1e-6
                    {
                        cnt += 1;
                    }
                    if ((darts[k][0] as f64 - x4).powi(2) + (darts[k][1] as f64 - y4).powi(2)
                        - (r as f64).powi(2))
                    .abs()
                        < 1e-6
                    {
                        cnt += 1;
                    }
                }
                res = res.max(cnt);
            }
        }
        if res == 1 {
            return 1;
        }
        return res;
    }
}
