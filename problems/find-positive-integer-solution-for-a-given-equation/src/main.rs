fn main() {
    assert_eq!(
        Solution::find_solution(1, 5),
        vec![[1, 4], [2, 3], [3, 2], [4, 1]]
    );
    assert_eq!(Solution::find_solution(2, 5), vec![[1, 5], [5, 1]]);
}

struct Solution;

// This is the custom function interface.
// You should not implement it, or speculate about its implementation
struct CustomFunction;
impl CustomFunction {
    pub fn f(x: i32, y: i32) -> i32 {
        x + y
    }
}

impl Solution {
    pub fn find_solution(customfunction: &CustomFunction, z: i32) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        for x in 1..=1000 {
            for y in 1..=1000 {
                if customfunction.f(x, y) == z {
                    result.push(vec![x, y]);
                }
            }
        }
        return result;
    }
}
