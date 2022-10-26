fn main() {
    assert_eq!(Solution::mincost_to_hire_workers(vec![10,20,5], vec![70,50,30], 2), 105.00000);
    assert_eq!(Solution::mincost_to_hire_workers(vec![3,1,10,10,1], vec![4,8,2,2,7], 3), 30.66667);
}

struct Solution;
impl Solution {
    pub fn mincost_to_hire_workers(quality: Vec<i32>, wage: Vec<i32>, k: i32) -> f64 {
        return 0f64;
    }
}
