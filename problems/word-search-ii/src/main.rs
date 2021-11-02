fn main() {
    assert_eq!(
        Solution::find_words(
            vec![
                vec!['o', 'a', 'a', 'n'],
                vec!['e', 't', 'a', 'e'],
                vec!['i', 'h', 'k', 'r'],
                vec!['i', 'f', 'l', 'v']
            ],
            vec!["oath".to_string(), "pea".to_string(), "eat".to_string(), "rain".to_string()]
        ),
        vec!["eat", "oath"]
    );
}

pub struct Solution {}
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<Vec<ListNode>>>,
}

impl ListNode {
    #[inline]
    fn new(words: Vec<String>) -> Self {
        ListNode { next: None, val:0}
    }
}
impl Solution {
    fn helper(board: Vec<Vec<char>>, visited: &mut Vec<Vec<bool>>, i: usize, j: usize) {
        let m = board.len();
        let n = board[0].len();

        if i >= m || j >= n {
            return;
        }

        if visited[i][j] {
            return;
        }

        // let s = board[i][j];

        visited[i][j] = true;

        if i > 0 {
            Solution::helper(board.clone(), visited, i-1, j);
        }
        Solution::helper(board.clone(), visited, i+1, j);

        if j > 0 {
            Solution::helper(board.clone(), visited, i, j-1);
        }
        Solution::helper(board.clone(), visited, i, j+1);
        visited[i][j] = false;
    }
    pub fn find_words(board: Vec<Vec<char>>, words: Vec<String>) -> Vec<String> {

        /*
        oath
        o

        a
        t
        h
        node->item = oath
        node->children['o','a','t','h']

        oath
        o
        a
        t
        h
        [o][a][t][h] = oath
        */

        let root = ListNode::new(words);

        println!("root {:?}", root);

        let m = board.len();
        let n = board[0].len();

        let mut visited = vec![vec![false;n];m];

        for i in 0..m {
            for j in 0..n {
                Solution::helper(board.clone(), &mut visited, i, j);
            }
        }
        return vec![];
    }
}
