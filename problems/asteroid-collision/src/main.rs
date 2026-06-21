fn main() {
    assert_eq!(Solution::asteroid_collision(vec![5,10,-5]), vec![5,10]);
    assert_eq!(Solution::asteroid_collision(vec![8,-8]), vec![]);
    assert_eq!(Solution::asteroid_collision(vec![10,2,-5]), vec![10]);
}

struct Solution{}
impl Solution {
    pub fn asteroid_collision(asteroids: Vec<i32>) -> Vec<i32> {
        let mut stack: Vec<i32> = Vec::new();

        for asteroid in asteroids {
            let current = asteroid;
            let mut alive = true;

            while alive && current < 0 {
                let Some(&last) = stack.last() else {
                    break;
                };

                if last < 0 {
                    break;
                }

                if last < -current {
                    stack.pop();
                    continue;
                }

                if last == -current {
                    stack.pop();
                }

                alive = false;
            }

            if alive {
                stack.push(current);
            }
        }

        stack
    }
}
