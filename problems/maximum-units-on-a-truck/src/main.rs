fn main() {
    assert_eq!(
        Solution::maximum_units(vec![vec![1, 3], vec![2, 2], vec![3, 1]], 4),
        8
    );
    assert_eq!(
        Solution::maximum_units(vec![vec![5, 10], vec![2, 5], vec![4, 7], vec![3, 9]], 10),
        91
    );
}

struct Solution;
impl Solution {
    pub fn maximum_units(box_types: Vec<Vec<i32>>, truck_size: i32) -> i32 {
        let mut box_types = box_types;
        for i in 0..box_types.len() {
            for j in i + 1..box_types.len() {
                if box_types[i][1] < box_types[j][1] {
                    box_types.swap(i, j);
                }
            }
        }
        let mut truck_size = truck_size;
        let mut result = 0;
        for i in 0..box_types.len() {
            if truck_size >= box_types[i][0] {
                result += box_types[i][0] * box_types[i][1];
                truck_size -= box_types[i][0];
            } else {
                result += truck_size * box_types[i][1];
                break;
            }
        }
        result
    }
}
