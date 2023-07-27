fn main() {
    assert_eq!(
        Solution::num_equiv_domino_pairs(vec![vec![1, 2], vec![2, 1], vec![3, 4], vec![5, 6]]),
        1
    );
    assert_eq!(
        Solution::num_equiv_domino_pairs(vec![
            vec![1, 2],
            vec![1, 2],
            vec![1, 1],
            vec![1, 2],
            vec![2, 2]
        ]),
        3
    );
}

struct Solution;
impl Solution {
    pub fn num_equiv_domino_pairs(dominoes: Vec<Vec<i32>>) -> i32 {
        let mut count = [0; 100];
        let mut res = 0;
        for domino in dominoes {
            let num = domino[0].min(domino[1]) * 10 + domino[0].max(domino[1]);
            res += count[num as usize];
            count[num as usize] += 1;
        }
        return res;
    }
}
