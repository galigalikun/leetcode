fn main() {
    assert_eq!(
        Solution::find_all_people(6, vec![vec![1, 2, 5], vec![2, 3, 8], vec![1, 5, 10]], 1),
        vec![0, 1, 2, 3, 5]
    );
    assert_eq!(
        Solution::find_all_people(
            4,
            vec![[3, 1, 3], [1, 2, 2], [0, 3, 3]]
                .iter()
                .map(|&x| x.to_vec())
                .collect(),
            3
        ),
        vec![0, 1, 2, 3]
    );
}

struct Solution;
impl Solution {
    pub fn find_all_people(n: i32, meetings: Vec<Vec<i32>>, first_person: i32) -> Vec<i32> {
        let mut informed = vec![false; n as usize];
        informed[0] = true;
        informed[first_person as usize] = true;
        for meeting in meetings {
            let a = meeting[0] as usize;
            let b = meeting[1] as usize;
            if informed[a] || informed[b] {
                informed[a] = true;
                informed[b] = true;
            }
        }
        return informed
            .iter()
            .enumerate()
            .filter(|&(_, &info)| info)
            .map(|(i, _)| i as i32)
            .collect();
    }
}
