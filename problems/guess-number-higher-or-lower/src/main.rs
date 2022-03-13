fn main() {
    /*
        2
    1
        */
    unsafe {
        assert_eq!(Solution::guessNumber(2), 1);
    }
}
unsafe fn guess(num: i32) -> i32 {
    if 1 > num {
        return 1;
        //////1776200683
    } else if 1 < num {
        return -1;
    }
    return 0;
}
/**
 * Forward declaration of guess API.
 * @param  num   your guess
 * @return 	     -1 if num is lower than the guess number
 *			      1 if num is higher than the guess number
 *               otherwise return 0
 * unsafe fn guess(num: i32) -> i32 {}
 */
struct Solution {}
impl Solution {
    unsafe fn guessNumber(n: i32) -> i32 {
        let mut num = n;
        let mut min = 1;
        let mut max = std::i32::MAX - 1;
        loop {
            match guess(num) {
                1 => {
                    min = num;
                    num = num
                        + if (max - min) / 2 > 0 {
                            (max - min) / 2
                        } else {
                            1
                        };
                }
                -1 => {
                    max = num;
                    num = num
                        - if (max - min) / 2 > 0 {
                            (max - min) / 2
                        } else {
                            1
                        };
                }
                _ => return num,
            };
        }
    }
}
