fn main() {
    assert_eq!(Solution::num_buses_to_destination(vec![vec![1,2,7],vec![3,6,7]], 1, 6), 2);
    assert_eq!(Solution::num_buses_to_destination(vec![vec![7,12],vec![4,5,15],vec![6],vec![15,19],vec![9,12,13]], 15, 12), -1);
}

struct Solution{}
impl Solution {
    pub fn num_buses_to_destination(routes: Vec<Vec<i32>>, source: i32, target: i32) -> i32 {
        return -1;
    }
}
