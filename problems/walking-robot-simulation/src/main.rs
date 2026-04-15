fn main() {
    assert_eq!(Solution::robot_sim(vec![4,-1,3], vec![]), 25);
    assert_eq!(Solution::robot_sim(vec![4,-1,4,-2,4], vec![vec![2,4]]), 65);
    assert_eq!(Solution::robot_sim(vec![6,-1,-1,6], vec![]), 36);
}

struct Solution;
impl Solution {
    pub fn robot_sim(commands: Vec<i32>, obstacles: Vec<Vec<i32>>) -> i32 {
        let mut x = 0;
        let mut y = 0;
        let mut direction = 0; // 0: north, 1: east, 2: south, 3: west
        let mut max_distance = 0;
        let obstacle_set: std::collections::HashSet<(i32, i32)> = obstacles.into_iter().map(|o| (o[0], o[1])).collect();

        for command in commands {
            match command {
                -2 => direction = (direction + 3) % 4, // turn left
                -1 => direction = (direction + 1) % 4, // turn right
                _ => {
                    for _ in 0..command {
                        let (next_x, next_y) = match direction {
                            0 => (x, y + 1),
                            1 => (x + 1, y),
                            2 => (x, y - 1),
                            3 => (x - 1, y),
                            _ => unreachable!(),
                        };
                        if obstacle_set.contains(&(next_x, next_y)) {
                            break;
                        }
                        x = next_x;
                        y = next_y;
                        max_distance = max_distance.max(x * x + y * y);
                    }
                }
            }
        }

        max_distance
    }
}
