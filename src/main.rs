fn main() {
    let a = Solution::add_two_numbers(
        Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode {
                val: 4,
                next: Some(Box::new(ListNode { val: 3, next: None })),
            })),
        })),
        Some(Box::new(ListNode {
            val: 5,
            next: Some(Box::new(ListNode {
                val: 6,
                next: Some(Box::new(ListNode { val: 4, next: None })),
            })),
        })),
    );
    println!("Hello, world! {:?}", a);
}

// Definition for singly-linked list.
pub struct Solution {}

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
        let mut a: Option<Box<ListNode>> = None;
        let mut list1 = l1;
        let mut list2 = l2;
        let mut over = 0;
        loop {
            let mut n1 = 0;
            if let Some(data1) = list1 {
                n1 = data1.val;
                list1 = data1.next;
            }
            let mut n2 = 0;
            if let Some(data2) = list2 {
                n2 = data2.val;
                list2 = data2.next;
            }

            let num = n1 + n2 + over;

            over = num / 10;

            a = Some(Box::new(ListNode {
                val: num % 10,
                next: a,
            }));

            if list1 == None && list2 == None {
                if over > 0 {
                    a = Some(Box::new(ListNode {
                        val: over % 10,
                        next: a,
                    }));
                }
                break;
            }
        }
        let mut b: Option<Box<ListNode>> = None;
        loop {
            if let Some(data) = a {
                b = Some(Box::new(ListNode {
                    val: data.val,
                    next: b,
                }));
                a = data.next;
            } else {
                break;
            }
        }
        return b;
    }
}
