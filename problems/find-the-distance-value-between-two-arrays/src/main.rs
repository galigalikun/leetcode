fn main() {
    assert_eq!(
        Solution::find_the_distance_value(vec![4, 5, 8], vec![10, 9, 1, 8], 2),
        2
    );
    assert_eq!(
        Solution::find_the_distance_value(vec![1, 4, 2, 3], vec![-4, -3, 6, 10, 20, 30], 3),
        2
    );
    assert_eq!(
        Solution::find_the_distance_value(vec![2, 1, 100, 3], vec![-5, -2, 10, -3, 7], 6),
        1
    );
}

struct Solution;
impl Solution {
    pub fn find_the_distance_value(arr1: Vec<i32>, arr2: Vec<i32>, d: i32) -> i32 {
        let mut count = 0;
        for i in 0..arr1.len() {
            let mut found = false;
            for j in 0..arr2.len() {
                if (arr1[i] - arr2[j]).abs() <= d {
                    found = true;
                    break;
                }
            }
            if !found {
                count += 1;
            }
        }
        return count;
    }
}
