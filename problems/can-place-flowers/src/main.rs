fn main() {
    assert_eq!(Solution::can_place_flowers(vec![1, 0, 0, 0, 1], 1), true);
    assert_eq!(Solution::can_place_flowers(vec![1, 0, 0, 0, 1], 2), false);
    assert_eq!(
        Solution::can_place_flowers(vec![1, 0, 0, 0, 0, 1], 2),
        false
    );
    assert_eq!(
        Solution::can_place_flowers(vec![1, 0, 0, 0, 1, 0, 0], 2),
        true
    );
    assert_eq!(Solution::can_place_flowers(vec![0, 1, 0], 1), false);
    assert_eq!(Solution::can_place_flowers(vec![0], 1), true);
}

struct Solution {}
impl Solution {
    pub fn can_place_flowers(flowerbed: Vec<i32>, n: i32) -> bool {
        if n == 0 {
            return true;
        }
        let mut board = flowerbed;
        let mut num = n;
        for i in 0..board.len() {
            if board[i] == 0 {
                if i == 0 || (i > 0 && board[i - 1] == 0) {
                    if i + 1 < board.len() && board[i + 1] == 0 {
                        board[i] = 1;
                        num -= 1;
                        if num == 0 {
                            return true;
                        }
                    } else if i + 1 == board.len() {
                        board[i] = 1;
                        num -= 1;
                        if num == 0 {
                            return true;
                        }
                    }
                }
            }
        }
        return false;
    }
}
