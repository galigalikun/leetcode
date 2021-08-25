fn main() {
    assert_eq!(
        Solution::reconstruct_queue(vec![
            vec![7, 0],
            vec![4, 4],
            vec![7, 1],
            vec![5, 0],
            vec![6, 1],
            vec![5, 2]
        ]),
        vec![[5, 0], [7, 0], [5, 2], [6, 1], [4, 4], [7, 1]]
    );

    assert_eq!(
        Solution::reconstruct_queue(vec![
            vec![6, 0],
            vec![5, 0],
            vec![4, 0],
            vec![3, 2],
            vec![2, 2],
            vec![1, 4]
        ]),
        vec![[4, 0], [5, 0], [2, 2], [3, 2], [1, 4], [6, 0]]
    );
}

pub struct Solution {}
// https://evelynn.gitbooks.io/google-interview/content/queue-reconstruction-by-height.html
impl Solution {
    pub fn reconstruct_queue(people: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut pp = people;
        pp.sort_by(|a, b| {
            if a[0] == b[0] {
                a[1].cmp(&b[1])
            } else {
                b[0].cmp(&a[0])
            }
        });
        let mut tmp = vec![];
        for i in 0..pp.len() {
            tmp.insert(pp[i][1] as usize, pp[i].clone());
        }
        let mut result = vec![vec![0; 2]; pp.len()];
        for i in 0..tmp.len() {
            result[i][0] = tmp[i][0];
            result[i][1] = tmp[i][1];
        }
        return result;
    }
}
