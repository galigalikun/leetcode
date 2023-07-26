fn main() {
    assert_eq!(
        Solution::smallest_sufficient_team(
            vec![
                "java".to_string(),
                "nodejs".to_string(),
                "reactjs".to_string()
            ],
            vec![
                vec!["java".to_string()],
                vec!["nodejs".to_string()],
                vec!["nodejs".to_string(), "reactjs".to_string()]
            ]
        ),
        vec![0, 2]
    );
    assert_eq!(
        Solution::smallest_sufficient_team(
            vec![
                "algorithms".to_string(),
                "math".to_string(),
                "java".to_string(),
                "reactjs".to_string(),
                "csharp".to_string(),
                "aws".to_string()
            ],
            vec![
                vec![
                    "algorithms".to_string(),
                    "math".to_string(),
                    "java".to_string()
                ],
                vec![
                    "algorithms".to_string(),
                    "math".to_string(),
                    "reactjs".to_string()
                ],
                vec!["java".to_string(), "csharp".to_string(), "aws".to_string()],
                vec!["reactjs".to_string(), "csharp".to_string()],
                vec!["csharp".to_string(), "math".to_string()],
                vec!["aws".to_string(), "java".to_string()]
            ]
        ),
        vec![1, 2]
    );
}

struct Solution;
impl Solution {
    pub fn smallest_sufficient_team(req_skills: Vec<String>, people: Vec<Vec<String>>) -> Vec<i32> {
        let mut skills = vec![0; 1 << req_skills.len()];
        let mut parents = vec![vec![]; 1 << req_skills.len()];
        for (i, p) in people.iter().enumerate() {
            let mut skill = 0;
            for s in p {
                skill |= 1 << req_skills.iter().position(|x| x == s).unwrap();
            }
            for j in 0..skills.len() {
                let next = j | skill;
                if skills[next] == 0 || skills[next] > skills[j] + 1 {
                    skills[next] = skills[j] + 1;
                    parents[next] = parents[j].clone();
                    parents[next].push(i);
                }
            }
        }
        return parents[skills.len() - 1]
            .iter()
            .map(|x| *x as i32)
            .collect();
    }
}
