fn main() {
    assert_eq!(Solution::calculate("10 + 12".to_string()), 2);
}

struct Solution {}
#[derive(Debug, PartialEq, Eq)]
pub enum NodeKind {
    NdAdd, // +
    NdSub, // -
    NdMul, // *
    NdDiv, // /
    NdNum, // 整数
}
#[derive(Debug, PartialEq, Eq)]
pub struct Node {
    pub kind: NodeKind,
    pub left: Option<Box<Node>>,
    pub right: Option<Box<Node>>,
    pub val: i32,
}
impl Node {
    #[inline]
    fn new(kind: NodeKind, left: Option<Box<Node>>, right: Option<Box<Node>>) -> Self {
        Node {
            kind: kind,
            left: left,
            right: right,
            val: 0,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum TokenKind {
    TkReserved, // 記号
    TkNum,      // 整数トークン
    TkEof,      // 入力の終わりを表すトークン
}
#[derive(Debug, PartialEq, Eq)]
pub struct Token {
    pub kind: TokenKind,
    pub next: Option<Box<Token>>,
    pub val: i32,
    pub str: String,
}



impl Solution {
    fn consume(token: &mut Option<Box<Token>>, op: char) -> bool {
        if let Some(t) = token {
            if t.kind != TokenKind::TkReserved || t.str.chars().nth(0) != Some(op) {
                return false;
            }
            *token = t.next;
        }
        return false;
    }
    fn expect(op: char) {}
    fn expr(token: &mut Option<Box<Token>>, index: &mut usize) -> Option<Box<Node>> {
        let mut node: Option<Box<Node>> = Solution::mul(token);

        loop {
            if Solution::consume(token, '+') {
                node = Some(Box::new(Node::new(NodeKind::NdAdd, node, Solution::mul(token))));
            } else if Solution::consume(token, '-') {
                node = Some(Box::new(Node::new(NodeKind::NdSub, node, Solution::mul(token))));
            } else {
                return node;
            }
        }
    }
    fn mul(token: &mut Option<Box<Token>>) -> Option<Box<Node>> {
        let mut node: Option<Box<Node>> = Solution::primary(token);

        loop {
            if Solution::consume(token, '*') {
                node = Some(Box::new(Node::new(
                    NodeKind::NdMul,
                    node,
                    Solution::primary(token),
                )));
            } else if Solution::consume(token, '/') {
                node = Some(Box::new(Node::new(
                    NodeKind::NdDiv,
                    node,
                    Solution::primary(token),
                )));
            } else {
                return node;
            }
        }
    }
    fn expect_number() -> i32 {
        return 0;
    }
    fn primary(token: &mut Option<Box<Token>>) -> Option<Box<Node>> {
        if Solution::consume(token, '(') {
            let node = Solution::expr(token, &mut 1);
            Solution::expect(')');
            return node;
        }

        return Some(Box::new(Node {
            kind: NodeKind::NdNum,
            left: None,
            right: None,
            val: Solution::expect_number(),
        }));
    }
    pub fn calculate(s: String) -> i32 {
        let mut token: Option<Box<Token>> = None;
        let mut index = 0;
        while index < s.len() {
            match s.bytes().nth(index) {
                Some(b'+') | Some(b'-') | Some(b'*') | Some(b'/') | Some(b'(') | Some(b')') => {
                    token = Some(Box::new(Token {
                        kind: TokenKind::TkReserved,
                        next: token,
                        val: 0,
                        str: s[index..index + 1].to_string(),
                    }));
                    index += 1;
                }
                Some(b'0') | Some(b'1') | Some(b'2') | Some(b'3') | Some(b'4') | Some(b'5')
                | Some(b'6') | Some(b'7') | Some(b'8') | Some(b'9') => {
                    let p = &s[index..].find(|c| !char::is_numeric(c)).unwrap_or(s.len());
                    token = Some(Box::new(Token {
                        kind: TokenKind::TkNum,
                        next: token,
                        val: s[index..*p].parse::<i32>().unwrap(),
                        str: s[index..*p].to_string(),
                    }));
                    index += *p;
                }
                _ => {
                    index += 1;
                }
            }
        }

        println!("debug {:?}", token);
        return 0;
    }
}
