fn main() {
    assert_eq!(Solution::valid_tic_tac_toe(vec!["O  ".to_string(),"   ".to_string(),"   ".to_string()]), false);
    assert_eq!(Solution::valid_tic_tac_toe(vec!["XOX".to_string()," X ".to_string(),"   ".to_string()]), false);
    assert_eq!(Solution::valid_tic_tac_toe(vec!["XOX".to_string(),"O O".to_string(),"XOX".to_string()]), true);
}

struct Solution{}
impl Solution {
    pub fn valid_tic_tac_toe(board: Vec<String>) -> bool {
        return false;
    }
}
