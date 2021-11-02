fn main() {
    // assert_eq!(Solution::find_min(vec![3, 4, 5, 1, 2]), 1);
    // assert_eq!(Solution::find_min(vec![4, 5, 6, 7, 0, 1, 2]), 0);
    assert_eq!(Solution::find_min(vec![11, 13, 15, 17]), 11);
}

pub struct Solution {}
impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        /*
        [a[0], a[1], a[2], ..., a[n-1]]
        [a[n-1], a[0], a[1], a[2], ..., a[n-2]]

        [3,4,5,1,2]
        [1,2,3,4,5]
        [5,1,2,3,4] 1
        [4,5,1,2,3] 2
        [3,4,5,1,2] 3
        [2,3,4,5,1] 4
        [1,2,3,4,5] 5

        [3,4,5,1,2]
         0 1 2 3 4

         index 3 -> 1
        */

        let ori = nums.clone();

        let mut work = nums;
        work.sort();
        for i in 0..work.len() {
            if let Some(p) = work.pop() {
                work.insert(0, p);
            }
            if ori == work {
                if ori.len() > (i + 1) {
                    return ori[i + 1];
                } else {
                    return ori[0];
                }
            }
        }
        return 0;
    }
}
