fn main() {
    assert_eq!(
        Solution::add_two_numbers(
            Some(Box::new(ListNode {
                val: 7,
                next: Some(Box::new(ListNode {
                    val: 2,
                    next: Some(Box::new(ListNode {
                        val: 4,
                        next: Some(Box::new(ListNode { val: 3, next: None }))
                    }))
                }))
            })),
            Some(Box::new(ListNode {
                val: 5,
                next: Some(Box::new(ListNode {
                    val: 6,
                    next: Some(Box::new(ListNode { val: 4, next: None }))
                }))
            }))
        ),
        Some(Box::new(ListNode {
            val: 7,
            next: Some(Box::new(ListNode {
                val: 8,
                next: Some(Box::new(ListNode {
                    val: 0,
                    next: Some(Box::new(ListNode { val: 7, next: None }))
                }))
            }))
        }))
    );

    assert_eq!(
        Solution::add_two_numbers(
            Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 4,
                    next: Some(Box::new(ListNode { val: 3, next: None }))
                }))
            })),
            Some(Box::new(ListNode {
                val: 5,
                next: Some(Box::new(ListNode {
                    val: 6,
                    next: Some(Box::new(ListNode { val: 4, next: None }))
                }))
            }))
        ),
        Some(Box::new(ListNode {
            val: 8,
            next: Some(Box::new(ListNode {
                val: 0,
                next: Some(Box::new(ListNode { val: 7, next: None }))
            }))
        }))
    );

    assert_eq!(
        Solution::add_two_numbers(
            Some(Box::new(ListNode { val: 0, next: None })),
            Some(Box::new(ListNode { val: 0, next: None }))
        ),
        Some(Box::new(ListNode { val: 0, next: None }))
    );
}

pub struct Solution {}
// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut ll1 = l1;
        let mut ll2 = l2;
        let mut lst1 = vec![];
        let mut lst2 = vec![];
        loop {
            if let Some(l) = ll1 {
                lst1.push(l.val);
                ll1 = l.next;
            } else {
                break;
            }
        }
        loop {
            if let Some(l) = ll2 {
                lst2.push(l.val);
                ll2 = l.next;
            } else {
                break;
            }
        }

        let mut num = vec![];
        let mut div = 0;
        for i in 0..std::cmp::max(lst1.len(), lst2.len()) {
            let idx1 = lst1.len() as i32 - i as i32 - 1;
            let idx2 = lst2.len() as i32 - i as i32 - 1;
            let a1 = if idx1 >= 0 { lst1[idx1 as usize] } else { 0 };
            let a2 = if idx2 >= 0 { lst2[idx2 as usize] } else { 0 };
            let sum = div + a1 + a2;
            div = sum / 10;
            num.push(sum % 10);
        }

        if div > 0 {
            num.push(div);
        }

        let mut result: Option<Box<ListNode>> = None;
        for n in num {
            result = Some(Box::new(ListNode {
                val: n,
                next: result,
            }));
        }

        return result;
    }
}
