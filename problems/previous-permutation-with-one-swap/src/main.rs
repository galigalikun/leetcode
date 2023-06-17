fn main() {
    assert_eq!(Solution::prev_perm_opt1(vec![3,2,1]), vec![3,1,2]);
    assert_eq!(Solution::prev_perm_opt1(vec![1,1,5]), vec![1,1,5]);
    assert_eq!(Solution::prev_perm_opt1(vec![1,9,4,6,7]), vec![1,7,4,6,9]);
}

struct Solution;
impl Solution {
    pub fn prev_perm_opt1(arr: Vec<i32>) -> Vec<i32> {
        let mut arr = arr;
        let mut i = arr.len() - 1;
        while i > 0 && arr[i - 1] <= arr[i] {
            i -= 1;
        }
        if i == 0 {
            return vec![];
        }
        let mut j = arr.len() - 1;
        while arr[j] >= arr[i - 1] {
            j -= 1;
        }
        arr.swap(i - 1, j);
        arr[i..].reverse();
        return arr;
    }
}
