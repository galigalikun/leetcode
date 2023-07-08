fn main() {
    assert_eq!(Solution::find_in_mountain_array(3, vec![1,2,3,4,5,3,1]), 2);
    assert_eq!(Solution::find_in_mountain_array(3, vec![0,1,2,4,2,1]), -1);
}

struct Solution;

// This is the MountainArray's API interface.
// You should not implement it, or speculate about its implementation
struct MountainArray;
impl MountainArray {
    fn get(index:i32)->i32;
    fn length()->i32;
};


impl Solution {
    pub fn find_in_mountain_array(target: i32, mountainArr: &MountainArray) -> i32 {
        return -1;
    }
}
