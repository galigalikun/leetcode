fn main() {
    assert_eq!(
        Solution::best_coordinate(vec![vec![1, 2, 5], vec![2, 1, 7], vec![3, 1, 9]], 2),
        vec![2, 1]
    );
    assert_eq!(
        Solution::best_coordinate(vec![vec![23, 11, 21]], 9),
        vec![23, 11]
    );
    assert_eq!(
        Solution::best_coordinate(vec![vec![1, 2, 13], vec![2, 1, 7], vec![0, 1, 9]], 2),
        vec![1, 2]
    );
}

struct Solution;
impl Solution {
    pub fn best_coordinate(towers: Vec<Vec<i32>>, radius: i32) -> Vec<i32> {
        let mut max_q = 0;
        let mut max_x = 0;
        let mut max_y = 0;
        for x in 0..51 {
            for y in 0..51 {
                let mut q = 0;
                for tower in &towers {
                    let d = ((tower[0] - x).pow(2) + (tower[1] - y).pow(2)) as f64;
                    if d <= radius as f64 * radius as f64 {
                        q += (tower[2] as f64 / (1.0 + d.sqrt())).floor() as i32;
                    }
                }
                if q > max_q {
                    max_q = q;
                    max_x = x;
                    max_y = y;
                }
            }
        }
        if max_q > 0 {
            return vec![max_x, max_y];
        }
        return vec![0, 0];
    }
}
