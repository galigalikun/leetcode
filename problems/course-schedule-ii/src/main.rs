fn main() {
    assert_eq!(Solution::find_order(2, vec![vec![1, 0]]), vec![0, 1]);
    assert_eq!(
        Solution::find_order(4, vec![vec![1, 0], vec![2, 0], vec![3, 1], vec![3, 2]]),
        vec![0, 1, 2, 3]
    );
}

struct Solution {}
// https://akrad.hatenablog.com/entry/2020/07/04/130024
use std::collections::HashMap;
use std::collections::VecDeque;
impl Solution {
    pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
        if num_courses <= 0 {
            return vec![];
        }

        let mut in_degree = vec![0; num_courses as usize];

        let mut course_map: HashMap<i32, Vec<i32>> = HashMap::new();

        for i in 0..prerequisites.len() {
            in_degree[prerequisites[i][0] as usize] += 1;

            if let Some(c) = course_map.get_mut(&prerequisites[i][1]) {
                c.push(prerequisites[i][0]);
            } else {
                course_map.insert(prerequisites[i][1], vec![prerequisites[i][0]]);
            }
        }

        let mut in_study: VecDeque<i32> = VecDeque::new();
        for i in 0..in_degree.len() {
            if in_degree[i] == 0 {
                in_study.push_back(i as i32);
            }
        }

        let mut finished_courses = vec![0; num_courses as usize];
        let mut finished_count = 0;

        while !in_study.is_empty() {
            if let Some(finished_course) = in_study.pop_front() {
                finished_courses[finished_count] = finished_course;
                finished_count += 1;

                if let Some(next_candidates) = course_map.get(&finished_course) {
                    for next_candidate in next_candidates {
                        in_degree[*next_candidate as usize] -= 1;
                        if in_degree[*next_candidate as usize] == 0 {
                            in_study.push_back(*next_candidate);
                        }
                    }
                } else {
                    continue;
                }
            }
        }

        if finished_count as i32 == num_courses {
            return finished_courses;
        }
        return vec![];
    }
}
