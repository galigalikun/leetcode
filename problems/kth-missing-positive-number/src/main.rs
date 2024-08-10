fn main() {
    assert_eq!(Solution::find_kth_positive(vec![2, 3, 4, 7, 11], 5), 9);
    assert_eq!(Solution::find_kth_positive(vec![1, 2, 3, 4], 2), 6);
}

struct Solution;
impl Solution {
    pub fn find_kth_positive(arr: Vec<i32>, k: i32) -> i32 {
        let mut i = 0;
        let mut j = 1;
        let mut p = 1;
        while i < arr.len() {
            if arr[i] != j {
                if k == p {
                    return j;
                }
                j += 1;
                p += 1;
                continue;
            }
            i += 1;
            j += 1;
        }
        while k != p {
            j += 1;
            p += 1;
        }
        j
    }
}
