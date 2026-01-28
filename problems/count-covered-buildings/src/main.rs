fn main() {
    assert_eq!(
        Solution::count_covered_buildings(
            3,
            vec![[1, 2], [2, 2], [3, 2], [2, 1], [2, 3]]
                .iter()
                .map(|v| v.to_vec())
                .collect()
        ),
        1
    );
    assert_eq!(
        Solution::count_covered_buildings(
            4,
            vec![[1, 1], [1, 2], [2, 1], [2, 2]]
                .iter()
                .map(|v| v.to_vec())
                .collect()
        ),
        0
    );
    assert_eq!(
        Solution::count_covered_buildings(
            5,
            vec![[1, 3], [3, 2], [3, 3], [3, 5], [5, 3]]
                .iter()
                .map(|v| v.to_vec())
                .collect()
        ),
        1
    );
}

struct Solution;
impl Solution {
    pub fn count_covered_buildings(n: i32, buildings: Vec<Vec<i32>>) -> i32 {
        let mut count = 0;
        for i in 0..n as usize {
            let mut covered = false;
            for j in 0..n as usize {
                if i != j {
                    if buildings[j][0] <= buildings[i][0] && buildings[j][1] <= buildings[i][1] {
                        if buildings[j][0] < buildings[i][0] || buildings[j][1] < buildings[i][1] {
                            covered = true;
                            break;
                        }
                    }
                }
            }
            if covered {
                count += 1;
            }
        }
        return count;
    }
}
