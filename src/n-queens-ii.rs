fn main() {
    assert_eq!(Solution::total_n_queens(4), 2);
    assert_eq!(Solution::total_n_queens(1), 1);
}

pub struct Solution {}
impl Solution {
    fn helper(result: &mut i32, n: i32, y: i32, left: i32, down: i32, right: i32) {
        if y == n {
            *result += 1;
        } else {
            let mut bitmap = ((1 << n) - 1) & !(left | down | right);
            while bitmap != 0 {
                let bit = -bitmap & bitmap;
                bitmap ^= bit;
                Solution::helper(
                    result,
                    n,
                    y + 1,
                    (left | bit) << 1,
                    down | bit,
                    (right | bit) >> 1,
                );
            }
        }
    }
    pub fn total_n_queens(n: i32) -> i32 {
        let mut result = 0;
        Solution::helper(&mut result, n, 0, 0, 0, 0);

        return result;
    }
}
