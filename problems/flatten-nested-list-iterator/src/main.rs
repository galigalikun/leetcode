#[derive(Debug, PartialEq, Eq)]
pub enum NestedInteger {
    Int(i32),
    List(Vec<NestedInteger>),
}
struct NestedIterator {
    data: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NestedIterator {
    fn new(nested_list: Vec<NestedInteger>) -> Self {
        let mut res = vec![];
        fn helper(list: Vec<NestedInteger>, res: &mut Vec<i32>) {
            for n in list {
                match n {
                    NestedInteger::Int(v) => res.push(v),
                    NestedInteger::List(v) => helper(v, res),
                }
            }
        }
        helper(nested_list, &mut res);

        NestedIterator { data: res }
    }

    fn next(&mut self) -> i32 {
        return self.data.remove(0);
    }

    fn has_next(&self) -> bool {
        return !self.data.is_empty();
    }
}

/**
 * Your NestedIterator object will be instantiated and called as such:
 * let obj = NestedIterator::new(nestedList);
 * let ret_1: i32 = obj.next();
 * let ret_2: bool = obj.has_next();
 */
fn main() {
    let mut obj = NestedIterator::new(vec![
        NestedInteger::List(vec![NestedInteger::Int(1), NestedInteger::Int(1)]),
        NestedInteger::Int(2),
        NestedInteger::List(vec![NestedInteger::Int(1), NestedInteger::Int(1)]),
    ]);

    let mut res = vec![];
    while obj.has_next() {
        res.push(obj.next());
    }

    assert_eq!(res, vec![1, 1, 2, 1, 1]);
}
