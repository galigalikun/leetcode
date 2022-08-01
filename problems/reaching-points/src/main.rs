fn main() {
    assert_eq!(Solution::reaching_points(1, 1, 3, 5), true);
    assert_eq!(Solution::reaching_points(1, 1, 2, 2), false);
    assert_eq!(Solution::reaching_points(1, 1, 1, 1), true);
}

struct Solution{}
impl Solution {
    pub fn reaching_points(sx: i32, sy: i32, tx: i32, ty: i32) -> bool {
        if sx == tx && sy == ty {
            return true;
        }
        
        return Solution::reaching_points(sx, sy+sx, tx, ty);
    }
}
