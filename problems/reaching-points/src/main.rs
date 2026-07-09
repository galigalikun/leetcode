fn main() {
    assert_eq!(Solution::reaching_points(1, 1, 3, 5), true);
    assert_eq!(Solution::reaching_points(1, 1, 2, 2), false);
    assert_eq!(Solution::reaching_points(1, 1, 1, 1), true);
}

struct Solution {}
impl Solution {
    pub fn reaching_points(sx: i32, sy: i32, tx: i32, ty: i32) -> bool {
        let (sx, sy) = (sx as i64, sy as i64);
        let (mut tx, mut ty) = (tx as i64, ty as i64);

        // Work backwards from (tx, ty): the forward moves are additive,
        // so reverse moves are repeated subtraction, optimized with modulo.
        while tx >= sx && ty >= sy {
            if tx == sx && ty == sy {
                return true;
            }

            if tx > ty {
                if ty == sy {
                    return (tx - sx) % ty == 0;
                }
                tx %= ty;
            } else {
                if tx == sx {
                    return (ty - sy) % tx == 0;
                }
                ty %= tx;
            }
        }

        false
    }
}
