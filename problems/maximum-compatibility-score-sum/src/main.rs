fn main() {
    assert_eq!(Solution::max_compatibility_sum(vec![vec![1,1,0],vec![1,0,1],vec![0,0,1]], vec![vec![1,0,0],vec![0,0,1],vec![1,1,0]]), 8);
    assert_eq!(Solution::max_compatibility_sum(vec![vec![0,0,0],vec![0,0,0],vec![0,0,1]], vec![vec![1,1,1],vec![1,1,1],vec![1,1,0]]), 3);
}

struct Solution;
impl Solution {
    pub fn max_compatibility_sum(students: Vec<Vec<i32>>, mentors: Vec<Vec<i32>>) -> i32 {
        let n = students.len();
        for i in 0..n {
            for j in 0..n {
                // Calculate compatibility score between student i and mentor j
                let mut score = 0;
                for k in 0..students[i].len() {
                    score += students[i][k] * mentors[j][k];
                }
                // Do something with the score
                // For example, you could print it or store it in a data structure
                println!("Compatibility score between student {} and mentor {}: {}", i, j, score);

            }
        }
        students.len() as i32 * mentors.len() as i32 // Placeholder return value
    }
}
