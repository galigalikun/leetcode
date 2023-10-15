fn main() {
    assert_eq!(Solution::circular_permutation(2, 3), vec![3, 2, 0, 1]);
    assert_eq!(
        Solution::circular_permutation(3, 2),
        vec![2, 6, 7, 5, 4, 0, 1, 3]
    );
}

struct Solution;
impl Solution {
    pub fn circular_permutation(n: i32, start: i32) -> Vec<i32> {
        let mut res = vec![];
        let mut gray_code = vec![0];
        for i in 0..n {
            let mut tmp = gray_code.clone();
            tmp.reverse();
            gray_code.push(1 << i);
            for j in tmp {
                gray_code.push((1 << i) | j);
            }
        }
        let mut start_index = 0;
        for i in 0..gray_code.len() {
            if gray_code[i] == start {
                start_index = i;
                break;
            }
        }
        for i in start_index..gray_code.len() {
            res.push(gray_code[i]);
        }
        for i in 0..start_index {
            res.push(gray_code[i]);
        }
        return res;
    }
}
