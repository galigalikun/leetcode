fn main() {
    assert_eq!(Solution::get_row(3), vec![1, 3, 3, 1]);
    assert_eq!(Solution::get_row(0), vec![1]);
    assert_eq!(Solution::get_row(1), vec![1, 1]);
}

struct Solution {}
impl Solution {
    pub fn get_row(row_index: i32) -> Vec<i32> {
        let mut result: Vec<Vec<i32>> = Vec::new();
        for i in 0..=row_index {
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
            if i == row_index {
                return work;
            } else {
                result.push(work);
            }
        }
        return vec![];
    }
}
