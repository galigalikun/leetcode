fn main() {
    let a = Solution::add_two_numbers(Some(Box::new(ListNode {
        val: 2,
        next: Some(Box::new(ListNode {
            val: 4,
            next: Some(Box::new(ListNode {
                val: 3,
                next: None
            }))
        }))
    })),Some(Box::new(ListNode {
        val: 5,
        next: Some(Box::new(ListNode {
            val: 6,
            next: Some(Box::new(ListNode {
                val: 4,
                next: None
            }))
        }))
    })));
    println!("Hello, world! {:?}", a);
}

// Definition for singly-linked list.
pub struct Solution {}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

impl Solution {
    fn number(l: Option<Box<ListNode>>) -> i32 {
        let mut n = 0;
        let mut list = l;
        let mut i = 0;
        loop {
            match list {
                Some(data) => {
                    n += i32::pow(10, i) * data.val;
                    list = data.next;
                    i+=1;
                }
                _ => {
                    break;
                }
            }
        }

        return n;
    }
    fn digit(n: i32) -> u32 {
        let mut d = 1;
        loop {
            if n/i32::pow(10, d) > 1 {
                d+=1;
            } else {
                break;
            }
        }
        return d;
    }
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let sum = Solution::number(l1) + Solution::number(l2);

        let mut a: Option<Box<ListNode>> = None;
        let d = Solution::digit(sum);
        for n in (0..d).rev() {
            if n == (d-1) {
                a = Some(Box::new(ListNode {
                    val: sum/i32::pow(10, n),
                    next: None
                }));
            } else {
                a = Some(Box::new(ListNode {
                    val: sum/i32::pow(10, n)%i32::pow(10, 2-n),
                    next: a
                }));
            }

        }
        return a;
    }
}
