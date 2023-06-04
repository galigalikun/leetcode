fn main() {
    assert_eq!(Solution::is_robot_bounded("GGLLGG".to_string()), true);
    assert_eq!(Solution::is_robot_bounded("GG".to_string()), false);
    assert_eq!(Solution::is_robot_bounded("GL".to_string()), true);
}

struct Solution;
impl Solution {
    pub fn is_robot_bounded(instructions: String) -> bool {
        let mut x = 0;
        let mut y = 0;
        let mut dir = 0;
        for c in instructions.chars() {
            match c {
                'G' => match dir {
                    0 => y += 1,
                    1 => x += 1,
                    2 => y -= 1,
                    3 => x -= 1,
                    _ => unreachable!(),
                },
                'L' => dir = (dir + 3) % 4,
                'R' => dir = (dir + 1) % 4,
                _ => unreachable!(),
            }
        }
        if x == 0 && y == 0 {
            return true;
        }
        if dir != 0 {
            return true;
        }
        return false;
    }
}
