fn main() {
    assert_eq!(
        Solution::survived_robots_healths(
            vec![5, 4, 3, 2, 1],
            vec![2, 17, 9, 15, 10],
            "RRRRR".to_string()
        ),
        vec![2, 17, 9, 15, 10]
    );
    assert_eq!(
        Solution::survived_robots_healths(
            vec![3, 5, 2, 6],
            vec![10, 10, 15, 12],
            "RLRL".to_string()
        ),
        vec![14]
    );
    assert_eq!(
        Solution::survived_robots_healths(
            vec![1, 2, 5, 6],
            vec![10, 10, 11, 11],
            "RLRL".to_string()
        ),
        vec![]
    );
}

struct Solution;
impl Solution {
    pub fn survived_robots_healths(
        positions: Vec<i32>,
        healths: Vec<i32>,
        directions: String,
    ) -> Vec<i32> {
        let mut robots = positions
            .into_iter()
            .zip(healths.into_iter())
            .zip(directions.into_bytes())
            .map(|((p, h), d)| (p, h, d))
            .collect::<Vec<_>>();
        robots.sort_unstable_by_key(|&(p, _, _)| p);
        let mut stack = Vec::new();
        for (p, h, d) in robots {
            if d == b'L' {
                let mut health = h;
                while let Some((_, h, d)) = stack.pop() {
                    if d == b'R' {
                        if h == health {
                            health = 0;
                            break;
                        } else if h > health {
                            health = 0;
                            stack.push((p, h - 1, d));
                            break;
                        } else {
                            health -= 1;
                        }
                    } else {
                        stack.push((p, h, d));
                    }
                }
                if health > 0 {
                    stack.push((p, health, d));
                }
            } else {
                stack.push((p, h, d));
            }
        }
        stack.into_iter().map(|(_, h, _)| h).collect()
    }
}
