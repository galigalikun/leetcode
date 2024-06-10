struct SubrectangleQueries {
    rectangle: Vec<Vec<i32>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl SubrectangleQueries {
    fn new(rectangle: Vec<Vec<i32>>) -> Self {
        return SubrectangleQueries {
            rectangle: rectangle,
        };
    }

    fn update_subrectangle(&mut self, row1: i32, col1: i32, row2: i32, col2: i32, new_value: i32) {
        for i in row1..=row2 {
            for j in col1..=col2 {
                self.rectangle[i as usize][j as usize] = new_value;
            }
        }
    }

    fn get_value(&self, row: i32, col: i32) -> i32 {
        for i in (0..self.rectangle.len()).rev() {
            if i as i32 == row {
                for j in (0..self.rectangle[i].len()).rev() {
                    if j as i32 == col {
                        return self.rectangle[i][j];
                    }
                }
            }
        }
        return 0;
    }
}

/**
 * Your SubrectangleQueries object will be instantiated and called as such:
 * let obj = SubrectangleQueries::new(rectangle);
 * obj.update_subrectangle(row1, col1, row2, col2, newValue);
 * let ret_2: i32 = obj.get_value(row, col);
 */
fn main() {
    let mut obj = SubrectangleQueries::new(vec![
        vec![1, 2, 1],
        vec![4, 3, 4],
        vec![3, 2, 1],
        vec![1, 1, 1],
    ]);
    let ret_2: i32 = obj.get_value(0, 2);
    assert_eq!(ret_2, 1);
    obj.update_subrectangle(0, 0, 3, 2, 5);
    let ret_2: i32 = obj.get_value(0, 2);
    assert_eq!(ret_2, 5);
    let ret_2: i32 = obj.get_value(3, 1);
    assert_eq!(ret_2, 5);
    obj.update_subrectangle(3, 0, 3, 2, 10);
    let ret_2: i32 = obj.get_value(3, 1);
    assert_eq!(ret_2, 10);
    let ret_2: i32 = obj.get_value(0, 2);
    assert_eq!(ret_2, 5);
}
