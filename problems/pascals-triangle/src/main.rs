fn main() {
    assert_eq!(
        Solution::generate(5),
        vec![
            vec![1],
            vec![1, 1],
            vec![1, 2, 1],
            vec![1, 3, 3, 1],
            vec![1, 4, 6, 4, 1]
        ]
    );
}

struct Solution {}
impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = Vec::new();
        for i in 0..num_rows {
            let mut work = Vec::with_capacity(i as usize + 1);
            for col in 0..=i {
                if col == 0 || col == i {
                    work.push(1);
                } else {
                    work.push(
                        result[i as usize - 1][col as usize - 1]
                            + result[i as usize - 1][col as usize],
                    );
                }
            }
            result.push(work);
        }

        return result;
    }
}
