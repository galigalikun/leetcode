fn main() {
    assert_eq!(
        Solution::generate_parenthesis(3),
        vec!["((()))", "(()())", "(())()", "()(())", "()()()"]
    );
    assert_eq!(Solution::generate_parenthesis(1), vec!["()"]);
    assert_eq!(Solution::generate_parenthesis(2), vec!["(())", "()()"]);
    assert_eq!(
        Solution::generate_parenthesis(4),
        vec![
            "(((())))", "((()()))", "((())())", "((()))()", "(()(()))", "(()()())", "(()())()",
            "(())(())", "(())()()", "()((()))", "()(()())", "()(())()", "()()(())", "()()()()"
        ]
    );
    // assert_eq!(
    //     Solution::generate_parenthesis(5),
    //     vec![
    //         "((((()))))",
    //         "(((()())))",
    //         "(((())()))",
    //         "(((()))())",
    //         "(((())))()",
    //         "((()(())))",
    //         "((()()()))",
    //         "((()())())",
    //         "((()()))()",
    //         "((())(()))",
    //         "((())()())",
    //         "((())())()",
    //         "((()))(())",
    //         "((()))()()",
    //         "(()((())))",
    //         "(()(()()))",
    //         "(()(())())",
    //         "(()(()))()",
    //         "(()()(()))",
    //         "(()()()())",
    //         "(()()())()",
    //         "(()())(())",
    //         "(()())()()",
    //         "(())((()))",
    //         "(())(()())",
    //         "(())(())()",
    //         "(())()(())",
    //         "(())()()()",
    //         "()(((())))",
    //         "()((()()))",
    //         "()((())())",
    //         "()((()))()",
    //         "()(()(()))",
    //         "()(()()())",
    //         "()(()())()",
    //         "()(())(())",
    //         "()(())()()",
    //         "()()((()))",
    //         "()()(()())",
    //         "()()(())()",
    //         "()()()(())",
    //         "()()()()()"
    //     ]
    // );
}

pub struct Solution {}
impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut result = Vec::new();

        /*
        1 -> 1
        2 -> 2
        3 -> 5
        4 -> 14
        5 -> 42
        ((((
        */
        let base = ['('].repeat(n as usize);
        {
            let mut b = base.clone();
            b.append(&mut [')'].repeat(n as usize));

            result.push(b.iter().collect::<String>());
        }
        if n > 2 {
            for i in 0..(n - 2) {
                for j in 0..(n - 1) {
                    let mut b = base.clone();
                    for k in 0..=j-i {
                        b.insert((n - (i+1) + k) as usize, ')');
                    }
                    for _k in 1..n - j + i {
                        b.push(')');
                    }
                    result.push(b.iter().collect::<String>());
                }
            }

            {
                let mut b = base.clone();

                b.insert(1, ')');
                for _i in 0..(n - 1) {
                    b.push(')');
                }

                result.push(b.iter().collect::<String>());
            }
        }
        if n > 1 {
            let mut b = base.clone();
            for i in 0..n {
                b.insert((i * 2 + 1) as usize, ')');
            }

            result.push(b.iter().collect::<String>());
        }
        return result;
    }
}
