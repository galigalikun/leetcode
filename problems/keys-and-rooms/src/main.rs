fn main() {
    assert_eq!(Solution::can_visit_all_rooms(vec![vec![1],vec![2],vec![3],vec![]]), true);
    assert_eq!(Solution::can_visit_all_rooms(vec![vec![1,3],vec![3,0,1],vec![2],vec![0]]), false);
}

struct Solution{}
impl Solution {
    pub fn can_visit_all_rooms(rooms: Vec<Vec<i32>>) -> bool {
        return false;
    }
}
