fn main() {
    assert_eq!(Solution::grid_illumination(5, vec![vec![0,0],vec![4,4]], vec![vec![1,1],vec![1,0]]), vec![1,0]);
    assert_eq!(Solution::grid_illumination(5, vec![vec![0,0],vec![4,4]], vec![vec![1,1],vec![1,1]]), vec![1,1]);
    assert_eq!(Solution::grid_illumination(5, vec![vec![0,0],vec![0,4]], vec![vec![0,4],vec![0,1],vec![1,4]]), vec![1,1,0]);
}

struct Solution;
impl Solution {
    pub fn grid_illumination(n: i32, lamps: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut result = vec![];
        for query in queries {
            let mut is_illuminated = false;
            for lamp in lamps.clone() {
                if lamp[0] == query[0] || lamp[1] == query[1] || (lamp[0] - lamp[1]) == (query[0] - query[1]) || (lamp[0] + lamp[1]) == (query[0] + query[1]) {
                    is_illuminated = true;
                    break;
                }
            }
            result.push(if is_illuminated { 1 } else { 0 });
        }
        result
    }
}
