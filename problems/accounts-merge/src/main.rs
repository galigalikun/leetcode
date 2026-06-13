use std::collections::HashMap;

fn main() {
    assert_accounts_merge(
        Solution::accounts_merge(vec![
            vec![
                "John".to_string(),
                "johnsmith@mail.com".to_string(),
                "john_newyork@mail.com".to_string(),
            ],
            vec![
                "John".to_string(),
                "johnsmith@mail.com".to_string(),
                "john00@mail.com".to_string(),
            ],
            vec!["Mary".to_string(), "mary@mail.com".to_string()],
            vec!["John".to_string(), "johnnybravo@mail.com".to_string()],
        ]),
        vec![
            vec![
                "John".to_string(),
                "john00@mail.com".to_string(),
                "john_newyork@mail.com".to_string(),
                "johnsmith@mail.com".to_string(),
            ],
            vec!["Mary".to_string(), "mary@mail.com".to_string()],
            vec!["John".to_string(), "johnnybravo@mail.com".to_string()],
        ],
    );

    assert_accounts_merge(
        Solution::accounts_merge(vec![
            vec![
                "Gabe".to_string(),
                "Gabe0@m.co".to_string(),
                "Gabe3@m.co".to_string(),
                "Gabe1@m.co".to_string(),
            ],
            vec![
                "Kevin".to_string(),
                "Kevin3@m.co".to_string(),
                "Kevin5@m.co".to_string(),
                "Kevin0@m.co".to_string(),
            ],
            vec![
                "Ethan".to_string(),
                "Ethan5@m.co".to_string(),
                "Ethan4@m.co".to_string(),
                "Ethan0@m.co".to_string(),
            ],
            vec![
                "Hanzo".to_string(),
                "Hanzo3@m.co".to_string(),
                "Hanzo1@m.co".to_string(),
                "Hanzo0@m.co".to_string(),
            ],
            vec![
                "Fern".to_string(),
                "Fern5@m.co".to_string(),
                "Fern1@m.co".to_string(),
                "Fern0@m.co".to_string(),
            ],
        ]),
        vec![
            vec![
                "Ethan".to_string(),
                "Ethan0@m.co".to_string(),
                "Ethan4@m.co".to_string(),
                "Ethan5@m.co".to_string(),
            ],
            vec![
                "Gabe".to_string(),
                "Gabe0@m.co".to_string(),
                "Gabe1@m.co".to_string(),
                "Gabe3@m.co".to_string(),
            ],
            vec![
                "Hanzo".to_string(),
                "Hanzo0@m.co".to_string(),
                "Hanzo1@m.co".to_string(),
                "Hanzo3@m.co".to_string(),
            ],
            vec![
                "Kevin".to_string(),
                "Kevin0@m.co".to_string(),
                "Kevin3@m.co".to_string(),
                "Kevin5@m.co".to_string(),
            ],
            vec![
                "Fern".to_string(),
                "Fern0@m.co".to_string(),
                "Fern1@m.co".to_string(),
                "Fern5@m.co".to_string(),
            ],
        ],
    );
}

fn assert_accounts_merge(actual: Vec<Vec<String>>, expected: Vec<Vec<String>>) {
    assert_eq!(normalize_accounts(actual), normalize_accounts(expected));
}

fn normalize_accounts(mut accounts: Vec<Vec<String>>) -> Vec<Vec<String>> {
    accounts.sort();
    accounts
}

struct Solution {}

impl Solution {
    pub fn accounts_merge(accounts: Vec<Vec<String>>) -> Vec<Vec<String>> {
        let mut union_find = UnionFind::new(accounts.len());
        let mut email_to_account = HashMap::new();

        for (account_index, account) in accounts.iter().enumerate() {
            for email in account.iter().skip(1) {
                if let Some(&other_account_index) = email_to_account.get(email) {
                    union_find.union(account_index, other_account_index);
                } else {
                    email_to_account.insert(email.clone(), account_index);
                }
            }
        }

        let mut grouped_emails: HashMap<usize, Vec<String>> = HashMap::new();
        for (email, account_index) in email_to_account {
            let root = union_find.find(account_index);
            grouped_emails.entry(root).or_default().push(email);
        }

        let mut merged_accounts = Vec::with_capacity(grouped_emails.len());
        for (root, mut emails) in grouped_emails {
            emails.sort();

            let mut merged_account = Vec::with_capacity(emails.len() + 1);
            merged_account.push(accounts[root][0].clone());
            merged_account.extend(emails);
            merged_accounts.push(merged_account);
        }

        merged_accounts
    }
}

struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
}

impl UnionFind {
    fn new(size: usize) -> Self {
        Self {
            parent: (0..size).collect(),
            rank: vec![0; size],
        }
    }

    fn find(&mut self, node: usize) -> usize {
        if self.parent[node] != node {
            self.parent[node] = self.find(self.parent[node]);
        }

        self.parent[node]
    }

    fn union(&mut self, left: usize, right: usize) {
        let left_root = self.find(left);
        let right_root = self.find(right);

        if left_root == right_root {
            return;
        }

        if self.rank[left_root] < self.rank[right_root] {
            self.parent[left_root] = right_root;
        } else if self.rank[left_root] > self.rank[right_root] {
            self.parent[right_root] = left_root;
        } else {
            self.parent[right_root] = left_root;
            self.rank[left_root] += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{assert_accounts_merge, Solution};

    #[test]
    fn merges_connected_accounts() {
        assert_accounts_merge(
            Solution::accounts_merge(vec![
                vec![
                    "John".to_string(),
                    "johnsmith@mail.com".to_string(),
                    "john_newyork@mail.com".to_string(),
                ],
                vec![
                    "John".to_string(),
                    "johnsmith@mail.com".to_string(),
                    "john00@mail.com".to_string(),
                ],
                vec!["Mary".to_string(), "mary@mail.com".to_string()],
                vec!["John".to_string(), "johnnybravo@mail.com".to_string()],
            ]),
            vec![
                vec![
                    "John".to_string(),
                    "john00@mail.com".to_string(),
                    "john_newyork@mail.com".to_string(),
                    "johnsmith@mail.com".to_string(),
                ],
                vec!["Mary".to_string(), "mary@mail.com".to_string()],
                vec!["John".to_string(), "johnnybravo@mail.com".to_string()],
            ],
        );
    }

    #[test]
    fn leaves_disconnected_accounts_separate() {
        assert_accounts_merge(
            Solution::accounts_merge(vec![
                vec![
                    "Gabe".to_string(),
                    "Gabe0@m.co".to_string(),
                    "Gabe3@m.co".to_string(),
                    "Gabe1@m.co".to_string(),
                ],
                vec![
                    "Kevin".to_string(),
                    "Kevin3@m.co".to_string(),
                    "Kevin5@m.co".to_string(),
                    "Kevin0@m.co".to_string(),
                ],
                vec![
                    "Ethan".to_string(),
                    "Ethan5@m.co".to_string(),
                    "Ethan4@m.co".to_string(),
                    "Ethan0@m.co".to_string(),
                ],
                vec![
                    "Hanzo".to_string(),
                    "Hanzo3@m.co".to_string(),
                    "Hanzo1@m.co".to_string(),
                    "Hanzo0@m.co".to_string(),
                ],
                vec![
                    "Fern".to_string(),
                    "Fern5@m.co".to_string(),
                    "Fern1@m.co".to_string(),
                    "Fern0@m.co".to_string(),
                ],
            ]),
            vec![
                vec![
                    "Ethan".to_string(),
                    "Ethan0@m.co".to_string(),
                    "Ethan4@m.co".to_string(),
                    "Ethan5@m.co".to_string(),
                ],
                vec![
                    "Gabe".to_string(),
                    "Gabe0@m.co".to_string(),
                    "Gabe1@m.co".to_string(),
                    "Gabe3@m.co".to_string(),
                ],
                vec![
                    "Hanzo".to_string(),
                    "Hanzo0@m.co".to_string(),
                    "Hanzo1@m.co".to_string(),
                    "Hanzo3@m.co".to_string(),
                ],
                vec![
                    "Kevin".to_string(),
                    "Kevin0@m.co".to_string(),
                    "Kevin3@m.co".to_string(),
                    "Kevin5@m.co".to_string(),
                ],
                vec![
                    "Fern".to_string(),
                    "Fern0@m.co".to_string(),
                    "Fern1@m.co".to_string(),
                    "Fern5@m.co".to_string(),
                ],
            ],
        );
    }
}
