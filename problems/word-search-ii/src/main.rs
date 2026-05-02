use std::collections::HashMap;

fn main() {
    let mut result = Solution::find_words(
        vec![
            vec!['o', 'a', 'a', 'n'],
            vec!['e', 't', 'a', 'e'],
            vec!['i', 'h', 'k', 'r'],
            vec!['i', 'f', 'l', 'v'],
        ],
        vec![
            "oath".to_string(),
            "pea".to_string(),
            "eat".to_string(),
            "rain".to_string(),
        ],
    );
    result.sort();
    assert_eq!(result, vec!["eat", "oath"]);

    let result2 = Solution::find_words(
        vec![vec!['a', 'b'], vec!['c', 'd']],
        vec!["abcb".to_string()],
    );
    assert_eq!(result2, Vec::<String>::new());
}

#[derive(Default)]
struct TrieNode {
    children: HashMap<char, TrieNode>,
    word: Option<String>,
}

struct Solution {}

impl Solution {
    pub fn find_words(board: Vec<Vec<char>>, words: Vec<String>) -> Vec<String> {
        let mut root = TrieNode::default();

        for word in &words {
            let mut node = &mut root;
            for ch in word.chars() {
                node = node.children.entry(ch).or_default();
            }
            node.word = Some(word.clone());
        }

        let m = board.len();
        let n = board[0].len();
        let mut board = board;
        let mut result = Vec::new();

        for i in 0..m {
            for j in 0..n {
                Self::dfs(&mut board, i, j, &mut root, &mut result);
            }
        }

        result
    }

    fn dfs(
        board: &mut Vec<Vec<char>>,
        i: usize,
        j: usize,
        node: &mut TrieNode,
        result: &mut Vec<String>,
    ) {
        let ch = board[i][j];
        if ch == '#' {
            return;
        }

        let child = match node.children.get_mut(&ch) {
            Some(c) => c,
            None => return,
        };

        if let Some(word) = child.word.take() {
            result.push(word);
        }

        board[i][j] = '#';

        let m = board.len();
        let n = board[0].len();

        if i > 0 {
            Self::dfs(board, i - 1, j, child, result);
        }
        if i + 1 < m {
            Self::dfs(board, i + 1, j, child, result);
        }
        if j > 0 {
            Self::dfs(board, i, j - 1, child, result);
        }
        if j + 1 < n {
            Self::dfs(board, i, j + 1, child, result);
        }

        board[i][j] = ch;
    }
}
