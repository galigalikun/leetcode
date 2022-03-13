fn main() {
    assert_eq!(Solution::can_measure_water(3, 5, 4), true);
    assert_eq!(Solution::can_measure_water(2, 6, 5), false);
    assert_eq!(Solution::can_measure_water(1, 2, 3), true);
    assert_eq!(Solution::can_measure_water(34, 5, 6), true);
    assert_eq!(Solution::can_measure_water(6, 9, 1), false);
}

struct Solution {}
// https://baihuqian.github.io/2018-08-13-water-and-jug-problem/
impl Solution {
    fn helper(jug1_capacity: i32, jug2_capacity: i32) -> i32 {
        let (mut a, mut b) = if jug1_capacity > jug2_capacity {
            (jug1_capacity, jug2_capacity)
        } else {
            (jug2_capacity, jug1_capacity)
        };
        while b != 0 {
            let tmp = b;
            b = a % b;
            a = tmp;
        }
        return a;
    }
    pub fn can_measure_water(jug1_capacity: i32, jug2_capacity: i32, target_capacity: i32) -> bool {
        if target_capacity == 0 {
            return true;
        }
        if jug1_capacity + jug2_capacity < target_capacity {
            return false;
        }

        return target_capacity % Solution::helper(jug1_capacity, jug2_capacity) == 0;
    }
}
