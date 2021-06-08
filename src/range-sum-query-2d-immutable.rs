struct NumMatrix {
    matrix: Vec<Vec<i32>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumMatrix {
    fn new(matrix: Vec<Vec<i32>>) -> Self {
        NumMatrix { matrix: matrix }
    }

    fn sum_region(&self, row1: i32, col1: i32, row2: i32, col2: i32) -> i32 {
        let mut sum = 0;
        for y in row1..=row2 {
            for x in col1..=col2 {
                sum += self.matrix[y as usize][x as usize];
            }
        }
        return sum;
    }
}

/**
 * Your NumMatrix object will be instantiated and called as such:
 * let obj = NumMatrix::new(matrix);
 * let ret_1: i32 = obj.sum_region(row1, col1, row2, col2);
 */
fn main() {
    let obj = NumMatrix::new(vec![
        vec![3, 0, 1, 4, 2],
        vec![5, 6, 3, 2, 1],
        vec![1, 2, 0, 1, 5],
        vec![4, 1, 0, 1, 7],
        vec![1, 0, 3, 0, 5],
    ]);
    assert_eq!(obj.sum_region(2, 1, 4, 3), 8);
    assert_eq!(obj.sum_region(1, 1, 2, 2), 11);
    assert_eq!(obj.sum_region(1, 2, 2, 4), 12);
}
