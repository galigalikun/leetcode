fn main() {
    assert_eq!(Solution::valid_mountain_array(vec![2, 1]), false);
    assert_eq!(Solution::valid_mountain_array(vec![3, 5, 5]), false);
    assert_eq!(Solution::valid_mountain_array(vec![0, 3, 2, 1]), true);
    assert_eq!(
        Solution::valid_mountain_array(vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0]),
        false
    );
}

struct Solution;
impl Solution {
    pub fn valid_mountain_array(arr: Vec<i32>) -> bool {
        if arr.len() < 3 {
            return false;
        }
        let mut a = false;
        for i in 0..arr.len() - 1 {
            if arr[i] == arr[i + 1] {
                return false;
            } else if !a && arr[i] < arr[i + 1] {
            } else if i > 0 && arr[i] > arr[i + 1] {
                a = true;
            } else {
                return false;
            }
        }
        return a;
    }
}
