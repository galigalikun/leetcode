fn main() {
    assert_eq!(Solution::get_strongest(vec![1, 2, 3, 4, 5], 2), vec![5, 1]);
    assert_eq!(Solution::get_strongest(vec![1, 1, 3, 5, 5], 2), vec![5, 5]);
    assert_eq!(
        Solution::get_strongest(vec![6, 7, 11, 7, 6, 8], 5),
        vec![11, 8, 6, 6, 7]
    );
}

struct Solution;
impl Solution {
    pub fn get_strongest(arr: Vec<i32>, k: i32) -> Vec<i32> {
        let mut arr = arr;
        arr.sort();
        let n = arr.len();
        let m = arr[(n - 1) / 2];
        let mut arr: Vec<(i32, i32)> = arr.into_iter().map(|x| (x, (x - m).abs())).collect();
        arr.sort_by(|a, b| {
            if a.1 == b.1 {
                b.0.cmp(&a.0)
            } else {
                b.1.cmp(&a.1)
            }
        });
        arr.truncate(k as usize);
        return arr.into_iter().map(|x| x.0).collect();
    }
}
