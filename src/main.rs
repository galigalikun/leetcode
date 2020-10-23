fn main() {
    let a = Solution::add_two_numbers(Some(Box::new(ListNode {
        val: 9,
        next: None
    })),Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 9,
            next: Some(Box::new(ListNode {
                val: 9,
                next: Some(Box::new(ListNode {
                val: 9,
                next: Some(Box::new(ListNode {
                val: 9,
                next: Some(Box::new(ListNode {
                val: 9,
                next: Some(Box::new(ListNode {
                val: 9,
                next: Some(Box::new(ListNode {
                val: 9,
                next: Some(Box::new(ListNode {
                val: 9,
                next: Some(Box::new(ListNode {
                val: 9,
                next: None
            }))
            }))
            }))
            }))
            }))
            }))
            }))
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
    fn number(l: Option<Box<ListNode>>) -> i64 {
        let mut n:i64 = 0;
        let mut list = l;
        let mut i = 0;
        loop {
            match list {

                Some(data) => {
                    n += i64::pow(10, i) * (data.val as i64);
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
    fn digit(n: i64) -> u32 {
        return n.to_string().len() as u32;
    }
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let sum = Solution::number(l1) + Solution::number(l2);

        let mut a: Option<Box<ListNode>> = None;
        let d = Solution::digit(sum);
        for n in (0..d).rev() {
            a = Some(Box::new(ListNode {
                val: (sum/i64::pow(10, n)%10) as i32,
                next: a
            }));
        }
        return a;
    }
}
