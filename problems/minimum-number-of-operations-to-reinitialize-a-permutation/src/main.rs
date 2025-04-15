fn main() {
    assert_eq!(Solution::reinitialize_permutation(2), 1);
    assert_eq!(Solution::reinitialize_permutation(4), 2);
    assert_eq!(Solution::reinitialize_permutation(6), 4);
}

struct Solution;
impl Solution {
    pub fn reinitialize_permutation(n: i32) -> i32 {
        let mut perm = (0..n).collect::<Vec<_>>();
        let target = (0..n).collect::<Vec<_>>();
        let mut count = 0;
        loop {
            count += 1;
            let mut new_perm = vec![0; n as usize];
            for i in 0..n {
                if i % 2 == 0 {
                    new_perm[i as usize] = perm[(i / 2) as usize];
                } else {
                    new_perm[i as usize] = perm[(n + i) as usize / 2 as usize];
                }
            }
            perm = new_perm;
            if perm == target {
                break;
            }
        }
        count
    }
}
