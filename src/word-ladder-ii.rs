fn main() {
    assert_eq!(
        Solution::find_ladders(
            "hit".to_string(),
            "cog".to_string(),
            vec![
                "hot".to_string(),
                "dot".to_string(),
                "dog".to_string(),
                "lot".to_string(),
                "log".to_string(),
                "cog".to_string()
            ]
        ),
        vec![
            [
                "hit".to_string(),
                "hot".to_string(),
                "dot".to_string(),
                "dog".to_string(),
                "cog".to_string()
            ],
            [
                "hit".to_string(),
                "hot".to_string(),
                "lot".to_string(),
                "log".to_string(),
                "cog".to_string()
            ]
        ]
    );
}

pub struct Solution {}

use std::collections::LinkedList;
#[derive(Debug)]
struct Node {
    val:String,
    depth:i32,
    prev:Option<Box<Node>>,
}
impl Solution {
    pub fn find_ladders(
        begin_word: String,
        end_word: String,
        word_list: Vec<String>,
    ) -> Vec<Vec<String>> {
        let mut result:Vec<Vec<String>> = vec![];
        if word_list.iter().find(|&s| s == &end_word) == None{
            return result;
        }

        let mut queue = LinkedList::new();
        queue.push_back(Node{
            val: begin_word,
            depth: 0,
            prev:None
        });

        let mut min_len = std::i32::MIN;
        while let Some(top) = queue.pop_front() {
            if result.len() > 0 && top.depth > min_len {
                return result;
            }
            let mut arr = top.val.chars().collect::<Vec<_>>();
            let mut i = 0;
            for c in top.val.chars() {
                println!("debug {}", c);
                for a in (b'a'..=b'z').map(|c| c as char).rev() {
                    if c == a {
                        continue;
                    }
                    arr[i] = a;

                    if arr.iter().collect::<String>() == end_word {
                        let mut work = vec![end_word.clone()];
                        let mut p = Some(Box::new(top));
                        loop {
                            if let Some(t) = p {
                                work.push(t.val);
                                p = t.prev;
                            } else {
                                break;
                            }
                        }
                        result.push(work.iter().rev().map(|&x| x).collect::<Vec<_>>());
                        if top.depth <= min_len {
                            min_len = top.depth;
                        } else {
                            return result;
                        }
                    }
                    println!("alphabet {}", arr.iter().collect::<String>());
                }
                i += 1;
            }
        }

        return result;
    }
}
