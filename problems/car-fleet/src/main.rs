fn main() {
    assert_eq!(Solution::car_fleet(12, vec![10,8,0,5,3], vec![2,4,1,1,3]), 3);
    assert_eq!(Solution::car_fleet(10, vec![3], vec![3]), 1);
    assert_eq!(Solution::car_fleet(100, vec![0,2,4], vec![4,2,1]), 1);
}

struct Solution;
impl Solution {
    pub fn car_fleet(target: i32, position: Vec<i32>, speed: Vec<i32>) -> i32 {
        return 0;
    }
}
