fn main() {
    assert_eq!(Solution::asteroid_collision(vec![5,10,-5]), vec![5,10]);
    assert_eq!(Solution::asteroid_collision(vec![8,-8]), vec![]);
    assert_eq!(Solution::asteroid_collision(vec![10,2,-5]), vec![10]);
}

struct Solution{}
impl Solution {
    pub fn asteroid_collision(asteroids: Vec<i32>) -> Vec<i32> {

        return vec![];
    }
}
