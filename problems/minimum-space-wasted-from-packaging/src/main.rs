fn main() {
    assert_eq!(
        Solution::min_wasted_space(vec![2, 3, 5], vec![vec![4, 8], vec![2, 8]]),
        6
    );
    assert_eq!(
        Solution::min_wasted_space(vec![2, 3, 5], vec![vec![1, 4], vec![2, 3], vec![3, 4]]),
        -1
    );
    assert_eq!(
        Solution::min_wasted_space(
            vec![3, 5, 8, 10, 11, 12],
            vec![vec![12], vec![11, 9], vec![10, 5, 14]]
        ),
        9
    );
}

struct Solution;
impl Solution {
    pub fn min_wasted_space(packages: Vec<i32>, boxes: Vec<Vec<i32>>) -> i32 {
        return if packages.is_empty() {
            0
        } else {
            let mut packages = packages;
            packages.sort_unstable();
            let total: i32 = packages.iter().sum();
            let mut min_waste = i32::MAX;

            for box_sizes in boxes {
                let mut box_sizes = box_sizes;
                box_sizes.sort_unstable();
                if box_sizes.last().unwrap() < &packages.last().unwrap() {
                    continue;
                }

                let mut waste = 0;
                let mut j = 0;
                for &size in &box_sizes {
                    let mut current_sum = 0;
                    while j < packages.len() && packages[j] <= size {
                        current_sum += packages[j];
                        j += 1;
                    }
                    waste += size * (j - waste as usize) - current_sum;
                }
                min_waste = min_waste.min(waste);
            }

            if min_waste == i32::MAX { -1 } else { min_waste }
        };
    }
}
