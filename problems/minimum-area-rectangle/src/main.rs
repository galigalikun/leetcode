use std::collections::HashMap;

fn main() {
    assert_eq!(
        Solution::min_area_rect(vec![
            vec![1, 1],
            vec![1, 3],
            vec![3, 1],
            vec![3, 3],
            vec![2, 2]
        ]),
        4
    );
    assert_eq!(
        Solution::min_area_rect(vec![
            vec![1, 1],
            vec![1, 3],
            vec![3, 1],
            vec![3, 3],
            vec![4, 1],
            vec![4, 3]
        ]),
        2
    );
}

struct Solution;
impl Solution {
    pub fn min_area_rect(points: Vec<Vec<i32>>) -> i32 {
        let mut points_by_x: HashMap<i32, Vec<i32>> = HashMap::new();
        for point in points {
            points_by_x.entry(point[0]).or_default().push(point[1]);
        }

        let mut columns: Vec<(i32, Vec<i32>)> = points_by_x.into_iter().collect();
        columns.sort_unstable_by_key(|(x, _)| *x);

        let mut last_x_for_y_pair: HashMap<(i32, i32), i32> = HashMap::new();
        let mut min_area = i32::MAX;

        for (x, mut ys) in columns {
            ys.sort_unstable();
            ys.dedup();

            for i in 0..ys.len() {
                for j in i + 1..ys.len() {
                    let pair = (ys[i], ys[j]);
                    if let Some(prev_x) = last_x_for_y_pair.get(&pair) {
                        let area = (x - prev_x) * (ys[j] - ys[i]);
                        min_area = min_area.min(area);
                    }
                    last_x_for_y_pair.insert(pair, x);
                }
            }
        }

        if min_area == i32::MAX {
            0
        } else {
            min_area
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn finds_min_area_rectangle() {
        let points = vec![
            vec![1, 1],
            vec![1, 3],
            vec![3, 1],
            vec![3, 3],
            vec![2, 2],
        ];
        assert_eq!(Solution::min_area_rect(points), 4);
    }

    #[test]
    fn finds_smaller_rectangle_when_multiple_exist() {
        let points = vec![
            vec![1, 1],
            vec![1, 3],
            vec![3, 1],
            vec![3, 3],
            vec![4, 1],
            vec![4, 3],
        ];
        assert_eq!(Solution::min_area_rect(points), 2);
    }

    #[test]
    fn returns_zero_when_no_rectangle_exists() {
        let points = vec![vec![1, 1], vec![2, 2], vec![3, 3]];
        assert_eq!(Solution::min_area_rect(points), 0);
    }
}
