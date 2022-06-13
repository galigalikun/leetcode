fn main() {
    assert_eq!(Solution::count_of_atoms(String::from("H2O")), String::from("H2O"));
    assert_eq!(Solution::count_of_atoms(String::from("Mg(OH)2")), String::from("H2MgO2"));
}

struct Solution{}
impl Solution {
    pub fn count_of_atoms(formula: String) -> String {
        return formula;
    }
}
