fn main() {
    assert_eq!(Solution::can_win_nim(4), false);
    assert_eq!(Solution::can_win_nim(1), true);
    assert_eq!(Solution::can_win_nim(2), true);
}

pub struct Solution {}
impl Solution {
    pub fn can_win_nim(n: i32) -> bool {
        /*
        1 true
        2 true
        3 true
        4 false
        5 true -> 4 ->
        6 true -> 4 ->
        7 true -> 4 ->
        8 false ->
        9 true -> 8
        */
        if n%4 == 0 {
            return false;
        }
        return true;
    }
}
