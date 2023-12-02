fn main() {
    assert_eq!(Solution::find_best_value(vec![4, 9, 3], 10), 3);
    assert_eq!(Solution::find_best_value(vec![2, 3, 5], 10), 5);
    assert_eq!(
        Solution::find_best_value(vec![60864, 25176, 27249, 21296, 20204], 56803),
        11361
    );
    assert_eq!(Solution::find_best_value(vec![3, 4, 5, 6], 18), 6);
}

struct Solution;
impl Solution {
    pub fn find_best_value(arr: Vec<i32>, target: i32) -> i32 {
        let mut arr = arr;
        arr.sort();
        let mut sum = 0;
        let mut i = 0;
        while i < arr.len() {
            let avg = (target - sum) / (arr.len() - i) as i32;
            if avg <= arr[i] {
                let left = target - sum - avg * (arr.len() - i) as i32;
                if left <= (avg + 1) * (arr.len() - i) as i32 - target + sum {
                    return avg;
                } else {
                    return avg + 1;
                }
            }
            sum += arr[i];
            i += 1;
        }
        return arr[arr.len() - 1];
    }
}
