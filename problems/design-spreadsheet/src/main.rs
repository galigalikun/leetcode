use std::collections::HashMap;

struct Spreadsheet {
    cells: HashMap<String, i32>, // Using a HashMap to store cell values with cell names as keys
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Spreadsheet {

    fn new(_rows: i32) -> Self {
        Spreadsheet { cells: HashMap::new() }
    }

    fn set_cell(&mut self, cell: String, value: i32) {
        self.cells.insert(cell, value);
    }

    fn reset_cell(&mut self, cell: String) {
        self.cells.remove(&cell);
    }

    fn get_value(&self, formula: String) -> i32 {
        if formula.starts_with('=') {
            let expression = &formula[1..]; // Remove the '=' character
            let tokens: Vec<&str> = expression.split('+').collect();
            let mut sum = 0;
            for token in tokens {
                if let Ok(num) = token.parse::<i32>() {
                    sum += num; // If it's a number, add it to the sum
                } else {
                    sum += self.cells.get(token).cloned().unwrap_or(0); // If it's a cell reference, get its value
                }
            }
            return sum;
        }
        self.cells.get(&formula).cloned().unwrap_or(0)
    }
}

/**
 * Your Spreadsheet object will be instantiated and called as such:
 * let obj = Spreadsheet::new(rows);
 * obj.set_cell(cell, value);
 * obj.reset_cell(cell);
 * let ret_3: i32 = obj.get_value(formula);
 */
fn main() {
    let mut obj = Spreadsheet::new(3);
    assert_eq!(obj.get_value("=5+7".to_string()), 12);
    obj.set_cell("A1".to_string(), 10);
    assert_eq!(obj.get_value("=A1+6".to_string()), 16);
    obj.set_cell("B2".to_string(), 15);
    assert_eq!(obj.get_value("=A1+B2".to_string()), 25);
    obj.reset_cell("A1".to_string());
    assert_eq!(obj.get_value("=A1+B2".to_string()), 15);
}
