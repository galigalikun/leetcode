use std::collections::HashMap;

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
    assert_eq!(
        Solution::smallest_sufficient_team(
            vec!["a".to_string(), "b".to_string()],
            vec![
                vec!["a".to_string()],
                vec!["b".to_string()],
                vec!["a".to_string(), "b".to_string()]
            ]
        ),
        vec![2]
    );
    assert_eq!(
        Solution::smallest_sufficient_team(
            vec![],
            vec![vec!["a".to_string()]]
        ),
        vec![]
    );
}

struct Solution;
impl Solution {
    pub fn smallest_sufficient_team(req_skills: Vec<String>, people: Vec<Vec<String>>) -> Vec<i32> {
        if req_skills.is_empty() {
            return Vec::new();
        }

        let skill_to_bit: HashMap<&str, usize> = req_skills
            .iter()
            .enumerate()
            .map(|(index, skill)| (skill.as_str(), index))
            .collect();

        let total_masks = 1usize << req_skills.len();
        let mut person_masks = vec![0usize; people.len()];
        for (index, person) in people.iter().enumerate() {
            let mut mask = 0usize;
            for skill in person {
                if let Some(&bit) = skill_to_bit.get(skill.as_str()) {
                    mask |= 1usize << bit;
                }
            }
            person_masks[index] = mask;
        }

        let mut dp: Vec<Option<Vec<usize>>> = vec![None; total_masks];
        dp[0] = Some(Vec::new());

        for person_index in 0..people.len() {
            let person_mask = person_masks[person_index];
            if person_mask == 0 {
                continue;
            }

            for mask in 0..total_masks {
                if let Some(team) = &dp[mask] {
                    let next_mask = mask | person_mask;
                    let mut candidate = team.clone();
                    candidate.push(person_index);
                    match &dp[next_mask] {
                        Some(existing) if existing.len() <= candidate.len() => {}
                        _ => dp[next_mask] = Some(candidate),
                    }
                }
            }
        }

        dp[total_masks - 1]
            .clone()
            .unwrap_or_default()
            .into_iter()
            .map(|index| index as i32)
            .collect()
    }
}
