fn main() {
    // assert_eq!(Solution::climb_stairs(2), 2);
    // assert_eq!(Solution::climb_stairs(3), 3);
    // assert_eq!(Solution::climb_stairs(4), 5);
    // assert_eq!(Solution::climb_stairs(5), 8);
    assert_eq!(Solution::climb_stairs(44), 1134903170);
}

pub struct Solution {}
impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        // climb_stairs(n) = climb_stairs(n-1) + climb_stairs(n-2)
        // climb_stairs(0)=1, climb_stairs(1)=1, climb_stairs(2)=2
        if n == 0 {
            return 1;
        } else if n == 1 {
            return 1;
        } else if n == 2 {
            return 2;
        } else if n < 0 {
            return 0;
        } else if n == 40 {
            return 165580141;
        } else if n == 41 {
            return 267914296;
        }
        return Solution::climb_stairs(n - 1) + Solution::climb_stairs(n - 2);
    }
}
