fn main() {
    assert_eq!(Solution::count_triplets(vec![2, 3, 1, 6, 7]), 4);
    assert_eq!(Solution::count_triplets(vec![1, 1, 1, 1, 1]), 10);
}

struct Solution;
impl Solution {
    pub fn count_triplets(arr: Vec<i32>) -> i32 {
        let mut count = 0;
        for i in 0..arr.len() {
            let mut xor = arr[i];
            for j in (i + 1)..arr.len() {
                xor ^= arr[j];
                if xor == 0 {
                    count += j - i;
                }
            }
        }
        return count as i32;
    }
}
