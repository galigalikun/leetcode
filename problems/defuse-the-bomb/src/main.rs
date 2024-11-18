fn main() {
    assert_eq!(Solution::decrypt(vec![5, 7, 1, 4], 3), vec![12, 10, 16, 13]);
    assert_eq!(Solution::decrypt(vec![1, 2, 3, 4], 0), vec![0, 0, 0, 0]);
    assert_eq!(Solution::decrypt(vec![2, 4, 9, 3], -2), vec![12, 5, 6, 13]);
}

struct Solution;
impl Solution {
    pub fn decrypt(code: Vec<i32>, k: i32) -> Vec<i32> {
        let n = code.len();
        let mut res = vec![0; n];
        if k == 0 {
            return res;
        }
        for i in 0..n {
            let mut sum = 0;
            let mut j = 1;
            while j <= k.abs() as usize {
                let idx = if k > 0 {
                    (i + j) % n
                } else {
                    if i >= j {
                        i - j
                    } else {
                        n - j + i
                    }
                };
                sum += code[idx];
                j += 1;
            }
            res[i] = sum;
        }
        res
    }
}
