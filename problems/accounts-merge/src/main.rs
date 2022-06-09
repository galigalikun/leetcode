fn main() {
    assert_eq!(
        Solution::accounts_merge(vec![
            vec![
                "John".to_string(),
                "johnsmith@mail.com".to_string(),
                "john_newyork@mail.com".to_string()
            ],
            vec![
                "John".to_string(),
                "johnsmith@mail.com".to_string(),
                "john00@mail.com".to_string()
            ],
            vec!["Mary".to_string(), "mary@mail.com".to_string()],
            vec!["John".to_string(), "johnnybravo@mail.com".to_string()]
        ]),
        vec![
            vec![
                "John",
                "john00@mail.com",
                "john_newyork@mail.com",
                "johnsmith@mail.com"
            ],
            vec!["Mary", "mary@mail.com"],
            vec!["John", "johnnybravo@mail.com"]
        ]
    );
    assert_eq!(
        Solution::accounts_merge(vec![
            vec![
                "Gabe".to_string(),
                "Gabe0@m.co".to_string(),
                "Gabe3@m.co".to_string(),
                "Gabe1@m.co".to_string()
            ],
            vec![
                "Kevin".to_string(),
                "Kevin3@m.co".to_string(),
                "Kevin5@m.co".to_string(),
                "Kevin0@m.co".to_string()
            ],
            vec![
                "Ethan".to_string(),
                "Ethan5@m.co".to_string(),
                "Ethan4@m.co".to_string(),
                "Ethan0@m.co".to_string()
            ],
            vec![
                "Hanzo".to_string(),
                "Hanzo3@m.co".to_string(),
                "Hanzo1@m.co".to_string(),
                "Hanzo0@m.co".to_string()
            ],
            vec![
                "Fern".to_string(),
                "Fern5@m.co".to_string(),
                "Fern1@m.co".to_string(),
                "Fern0@m.co".to_string()
            ]
        ]),
        vec![
            vec!["Ethan", "Ethan0@m.co", "Ethan4@m.co", "Ethan5@m.co"],
            vec!["Gabe", "Gabe0@m.co", "Gabe1@m.co", "Gabe3@m.co"],
            vec!["Hanzo", "Hanzo0@m.co", "Hanzo1@m.co", "Hanzo3@m.co"],
            vec!["Kevin", "Kevin0@m.co", "Kevin3@m.co", "Kevin5@m.co"],
            vec!["Fern", "Fern0@m.co", "Fern1@m.co", "Fern5@m.co"]
        ]
    );
}

struct Solution {}
impl Solution {
    pub fn accounts_merge(accounts: Vec<Vec<String>>) -> Vec<Vec<String>> {
        let mut graph: Vec<Vec<usize>> = vec![vec![]; accounts.len()];
        let mut emails: Vec<Vec<String>> = vec![vec![]; accounts.len()];
        for (i, account) in accounts.iter().enumerate() {
            for email in account.iter() {
                emails[i].push(email.to_string());
            }
        }
        for (i, account) in accounts.iter().enumerate() {
            for email in account.iter() {
                let j = emails.iter().position(|x| x.contains(email)).unwrap();
                graph[i].push(j);
            }
        }
        let mut visited: Vec<bool> = vec![false; accounts.len()];
        let mut ans: Vec<Vec<String>> = vec![];
        for i in 0..accounts.len() {
            if visited[i] {
                continue;
            }
            let mut stack: Vec<usize> = vec![i];
            visited[i] = true;
            let mut curr: Vec<String> = vec![];
            while !stack.is_empty() {
                let j = stack.pop().unwrap();
                for k in graph[j].iter() {
                    if !visited[*k] {
                        visited[*k] = true;
                        stack.push(*k);
                    }
                }
                curr.extend_from_slice(&emails[j]);
            }
            ans.push(curr);
        }
        return ans;
    }
}
