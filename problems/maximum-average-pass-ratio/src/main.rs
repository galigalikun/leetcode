fn main() {
    assert_eq!(
        Solution::max_average_ratio(vec![vec![1, 2], vec![3, 5], vec![2, 2]], 2),
        0.78333
    );
    assert_eq!(
        Solution::max_average_ratio(vec![vec![2, 4], vec![3, 9], vec![4, 5], vec![1, 6]], 4),
        0.53485
    );
}

struct Solution;
impl Solution {
    pub fn max_average_ratio(classes: Vec<Vec<i32>>, extra_students: i32) -> f64 {
        let mut classes = classes;
        let mut extra_students = extra_students;
        let mut heap = std::collections::BinaryHeap::new();
        for class in classes.iter() {
            let (pass, total) = (class[0], class[1]);
            let ratio = pass as f64 / total as f64;
            let new_ratio = (pass + 1) as f64 / (total + 1) as f64;
            let diff = new_ratio - ratio;
            heap.push((diff, pass, total));
        }
        while extra_students > 0 {
            if let Some((diff, pass, total)) = heap.pop() {
                let new_pass = pass + 1;
                let new_total = total + 1;
                let new_ratio = new_pass as f64 / new_total as f64;
                let new_diff = (new_pass as f64 / new_total as f64) - (pass as f64 / total as f64);
                heap.push((new_diff, new_pass, new_total));
                extra_students -= 1;
            }
        }
        let mut total_ratio = 0.0;
        let mut total_classes = 0;
        for class in classes.iter() {
            let (pass, total) = (class[0], class[1]);
            total_ratio += pass as f64 / total as f64;
            total_classes += 1;
        }
        total_ratio /= total_classes as f64;
        total_ratio
    }
}
