fn main() {
    assert_eq!(Solution::min_integer("4321".to_string(), 4), "1342");
    assert_eq!(Solution::min_integer("100".to_string(), 1), "010");
    assert_eq!(Solution::min_integer("36789".to_string(), 1000), "36789");
}

struct Solution;
impl Solution {
    pub fn min_integer(num: String, k: i32) -> String {
        let mut num = num.chars().collect::<Vec<_>>();
        let mut k = k as usize;
        let n = num.len();
        let mut pos = vec![0; 10];
        let mut bit = vec![0; n + 1];
        for i in 0..n {
            pos[num[i] as usize - '0' as usize] = i;
        }
        for i in 1..=n {
            bit[i] = i as i32 & -(i as i32);
        }
        let mut i = 0;
        while i < n && k > 0 {
            let mut j = i + 1;
            while j <= n && j - i <= k {
                if pos[num[i] as usize - '0' as usize] >= i {
                    break;
                }
                j += bit[j - i] as usize;
            }
            let mut p = pos[num[i] as usize - '0' as usize];
            while p > i {
                let t = num[p];
                num[p] = num[p - 1];
                num[p - 1] = t;
                pos[num[p] as usize - '0' as usize] = p;
                pos[num[p - 1] as usize - '0' as usize] = p - 1;
                p -= 1;
                k -= 1;
            }
            i += 1;
        }
        return num.iter().collect();
    }
}
