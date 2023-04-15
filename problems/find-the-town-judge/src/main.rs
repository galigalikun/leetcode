fn main() {
    assert_eq!(Solution::find_judge(2, vec![vec![1, 2]]), 2);
    assert_eq!(Solution::find_judge(3, vec![vec![1, 3], vec![2, 3]]), 3);
    assert_eq!(
        Solution::find_judge(3, vec![vec![1, 3], vec![2, 3], vec![3, 1]]),
        -1
    );
}

struct Solution;
impl Solution {
    pub fn find_judge(n: i32, trust: Vec<Vec<i32>>) -> i32 {
        let mut trust_count = vec![0; n as usize];
        for t in trust {
            trust_count[t[0] as usize - 1] -= 1;
            trust_count[t[1] as usize - 1] += 1;
        }
        for i in 0..n {
            if trust_count[i as usize] == n - 1 {
                return i + 1;
            }
        }
        return -1;
    }
}
