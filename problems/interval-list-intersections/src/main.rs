fn main() {
    assert_eq!(
        Solution::interval_intersection(
            vec![vec![0, 2], vec![5, 10], vec![13, 23], vec![24, 25]],
            vec![vec![1, 5], vec![8, 12], vec![15, 24], vec![25, 26]]
        ),
        vec![[1, 2], [5, 5], [8, 10], [15, 23], [24, 24], [25, 25]]
    );
    assert_eq!(
        Solution::interval_intersection(vec![vec![1, 3], vec![5, 9]], vec![]),
        vec![vec![]]
    );
}

struct Solution;
impl Solution {
    pub fn interval_intersection(
        first_list: Vec<Vec<i32>>,
        second_list: Vec<Vec<i32>>,
    ) -> Vec<Vec<i32>> {
        let mut result = vec![];
        let mut i = 0;
        let mut j = 0;
        while i < first_list.len() && j < second_list.len() {
            let first = &first_list[i];
            let second = &second_list[j];
            if first[1] < second[0] {
                i += 1;
            } else if second[1] < first[0] {
                j += 1;
            } else {
                result.push(vec![first[0].max(second[0]), first[1].min(second[1])]);
                if first[1] < second[1] {
                    i += 1;
                } else {
                    j += 1;
                }
            }
        }
        result
    }
}
