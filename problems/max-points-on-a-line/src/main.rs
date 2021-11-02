fn main() {
    assert_eq!(
        Solution::max_points(vec![vec![1, 1], vec![2, 2], vec![3, 3]]),
        3
    );

    assert_eq!(
        Solution::max_points(vec![
            vec![1, 1],
            vec![3, 2],
            vec![5, 3],
            vec![4, 1],
            vec![2, 3],
            vec![1, 4]
        ]),
        4
    );
}

// https://www.programcreek.com/2014/04/leetcode-max-points-on-a-line-java/
// https://medium.com/@harycane/max-points-on-a-line-1e38f51d591f
pub struct Solution {}
use std::collections::HashMap;
impl Solution {
    fn generate_gcd(a: i32, b: i32) -> i32 {
        if b == 0 {
            return a;
        }
        return Solution::generate_gcd(b, a % b);
    }
    pub fn max_points(points: Vec<Vec<i32>>) -> i32 {
        let n = points.len();
        if n == 0 {
            return 0;
        }

        let mut result: HashMap<(i32, i32), i32> = HashMap::new();
        let mut max = 0;
        for i in 0..n {
            let mut duplicate = 1;
            let mut vertical = 0;
            for j in i + 1..n {
                if points[i][0] == points[j][0] {
                    if points[i][1] == points[j][1] {
                        duplicate += 1;
                    } else {
                        vertical += 1;
                    }
                } else {
                    let x = points[j][0] - points[i][0];
                    let y = points[j][1] - points[i][1];
                    let gcd = Solution::generate_gcd(x, y);
                    let slope = if gcd == 0 { (0, 0) } else { (x / gcd, y / gcd) };
                    if let Some(r) = result.get_mut(&slope) {
                        *r += 1;
                    } else {
                        result.insert(slope, 1);
                    }
                }
            }

            for (_, count) in &result {
                max = std::cmp::max(max, count + duplicate);
            }

            max = std::cmp::max(max, vertical + duplicate);
            result.clear();
        }

        return max;
    }
}
