fn main() {
    assert_eq!(
        Solution::count_trapezoids(
            vec![[1, 0], [2, 0], [3, 0], [2, 2], [3, 2]]
                .iter()
                .map(|&x| x.to_vec())
                .collect()
        ),
        3
    );
    assert_eq!(
        Solution::count_trapezoids(
            vec![[0, 0], [1, 0], [0, 1], [2, 1]]
                .iter()
                .map(|&x| x.to_vec())
                .collect()
        ),
        1
    );
}

struct Solution;
impl Solution {
    pub fn count_trapezoids(points: Vec<Vec<i32>>) -> i32 {
        let n = points.len();
        for i in 0..n {
            for j in i + 1..n {
                let mut parallel_count = 0;
                for k in 0..n {
                    if k != i && k != j {
                        for l in k + 1..n {
                            if l != i && l != j {
                                if (points[j][1] - points[i][1]) * (points[l][0] - points[k][0])
                                    == (points[l][1] - points[k][1]) * (points[j][0] - points[i][0])
                                {
                                    parallel_count += 1;
                                }
                            }
                        }
                    }
                }
                // Each pair of parallel lines can form a trapezoid with the original line
                if parallel_count > 0 {
                    return parallel_count;
                }
            }
        }
        return 0;
    }
}
