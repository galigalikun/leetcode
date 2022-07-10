fn main() {
    assert_eq!(
        Solution::contain_virus(vec![
            vec![0, 1, 0, 0, 0, 0, 0, 1],
            vec![0, 1, 0, 0, 0, 0, 0, 1],
            vec![0, 0, 0, 0, 0, 0, 0, 1],
            vec![0, 0, 0, 0, 0, 0, 0, 0]
        ]),
        10
    );

    assert_eq!(
        Solution::contain_virus(vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]]),
        4
    );
    assert_eq!(
        Solution::contain_virus(vec![
            vec![1, 1, 1, 0, 0, 0, 0, 0, 0],
            vec![1, 0, 1, 0, 1, 1, 1, 1, 1],
            vec![1, 1, 1, 0, 0, 0, 0, 0, 0]
        ]),
        13
    );
}

struct Solution {}
impl Solution {
    pub fn contain_virus(is_infected: Vec<Vec<i32>>) -> i32 {
        return -1;
    }
}
