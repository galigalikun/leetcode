fn main() {
    assert_eq!(
        Solution::relative_sort_array(
            vec![2, 3, 1, 3, 2, 4, 6, 7, 9, 2, 19],
            vec![2, 1, 4, 3, 9, 6]
        )
        .eq(&vec![2, 2, 2, 1, 4, 3, 3, 9, 6, 7, 19]),
        true
    );
    assert_eq!(
        Solution::relative_sort_array(vec![28, 6, 22, 8, 44, 17], vec![22, 28, 8, 6])
            .eq(&vec![22, 28, 8, 6, 17, 44]),
        true
    );
}
struct Solution;
impl Solution {
    pub fn relative_sort_array(arr1: Vec<i32>, arr2: Vec<i32>) -> Vec<i32> {
        let mut arr1 = arr1;
        let mut arr3 = Vec::new();
        for i in 0..arr2.len() {
            for j in 0..arr1.len() {
                if arr2[i] == arr1[j] {
                    arr3.push(arr1[j]);
                    arr1[j] = -1;
                }
            }
        }
        arr1.sort();
        for i in 0..arr1.len() {
            if arr1[i] != -1 {
                arr3.push(arr1[i]);
            }
        }
        return arr3;
    }
}
