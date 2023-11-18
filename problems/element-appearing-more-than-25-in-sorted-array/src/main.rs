fn main() {
    assert_eq!(
        Solution::find_special_integer(vec![1, 2, 2, 6, 6, 6, 6, 7, 10]),
        6
    );
    assert_eq!(Solution::find_special_integer(vec![1, 1]), 1);
}

struct Solution;
impl Solution {
    pub fn find_special_integer(arr: Vec<i32>) -> i32 {
        let mut count = 0;
        let len = arr.len() as f32;
        let mut prev = arr[0];
        for i in arr {
            if i == prev {
                count += 1;
            } else {
                prev = i;
                count = 1;
            }
            if count as f32 > len / 4.0 {
                return i;
            }
        }
        return 0;
    }
}
