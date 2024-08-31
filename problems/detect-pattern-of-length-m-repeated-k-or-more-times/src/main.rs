fn main() {
    assert_eq!(
        Solution::contains_pattern(vec![1, 2, 4, 4, 4, 4], 1, 3),
        true
    );
    assert_eq!(
        Solution::contains_pattern(vec![1, 2, 1, 2, 1, 1, 1, 3], 2, 2),
        true
    );
    assert_eq!(
        Solution::contains_pattern(vec![1, 2, 1, 2, 1, 3], 2, 3),
        false
    );
    assert_eq!(Solution::contains_pattern(vec![2, 2, 2, 2], 2, 3), false);
}

struct Solution;
impl Solution {
    pub fn contains_pattern(arr: Vec<i32>, m: i32, k: i32) -> bool {
        let n = arr.len();
        let m = m as usize;
        let k = k as usize;
        if n < m * k {
            return false;
        }
        for i in 0..n - m * k + 1 {
            let mut flag = true;
            for j in 0..m {
                for l in 0..k {
                    if arr[i + j] != arr[i + j + l * m] {
                        flag = false;
                        break;
                    }
                }
                if !flag {
                    break;
                }
            }
            if flag {
                return true;
            }
        }
        return false;
    }
}
