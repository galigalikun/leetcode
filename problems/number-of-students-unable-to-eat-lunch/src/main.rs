fn main() {
    assert_eq!(
        Solution::count_students(vec![1, 1, 0, 0], vec![0, 1, 0, 1]),
        0
    );
    assert_eq!(
        Solution::count_students(vec![1, 1, 1, 0, 0, 1], vec![1, 0, 0, 0, 1, 1]),
        3
    );
}

struct Solution;
impl Solution {
    pub fn count_students(students: Vec<i32>, sandwiches: Vec<i32>) -> i32 {
        let mut students = students;
        let mut sandwiches = sandwiches;
        let mut i = 0;
        loop {
            if students[0] == sandwiches[0] {
                students.remove(0);
                sandwiches.remove(0);
                i = 0;
            } else {
                let idx = students.remove(0);
                students.push(idx);
                i += 1;
            }
            if i == students.len() {
                break;
            }
        }
        students.len() as i32
    }
}
