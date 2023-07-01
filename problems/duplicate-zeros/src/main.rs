fn main() {
    let mut arr = vec![1, 0, 2, 3, 0, 4, 5, 0];
    Solution::duplicate_zeros(&mut arr);
    assert_eq!(arr, vec![1, 0, 0, 2, 3, 0, 0, 4]);

    arr = vec![1, 2, 3];
    Solution::duplicate_zeros(&mut arr);
    assert_eq!(arr, vec![1, 2, 3]);
}

struct Solution;
impl Solution {
    pub fn duplicate_zeros(arr: &mut Vec<i32>) {
        let mut i = 0;
        while i < arr.len() {
            if arr[i] == 0 {
                arr.insert(i, 0);
                arr.pop();
                i += 1;
            }
            i += 1;
        }
    }
}
