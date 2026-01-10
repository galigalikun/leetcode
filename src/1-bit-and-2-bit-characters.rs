fn main () {
    assert_eq!(Solution::is_one_bit_character(vec![1, 0, 0]), true);
    assert_eq!(Solution::is_one_bit_character(vec![1, 1, 1, 0]), false);
    assert_eq!(Solution::is_one_bit_character(vec![0]), true);
    assert_eq!(Solution::is_one_bit_character(vec![1, 1, 0]), true);
}

struct Solution{}
impl Solution {
    pub fn is_one_bit_character(bits: Vec<i32>) -> bool {
        // [0], [1, 0] [1, 1]
        // [1, 0, 0] -> [1, 0] , [0] true
        // [1, 1, 1, 0] -> [1, 1] , [1, 0] false
        // [1, 1, 0] -> [1] , [1, 0] or [1, 1] , [0] true
        // [0, 0, 0] -> [0] , [0] [0] true
        // [1, 1, 0, 0] -> [1, 1] [0] [0] -> true
        // [1, 0, 0, 0] -> [1, 0] [0] [0] -> true
        // [1, 0, 1, 0] -> [1, 0] [1, 0] -> false
        // [0, 1, 1, 0] -> [0] [1, 1] [0] -> true
        for i in (0..bits.len()-1).rev() {
            if bits[i] == 1 {
                return false;
            }
            if bits[i] == 0 {
                break;
            }
        }
        return true;
    }
}
