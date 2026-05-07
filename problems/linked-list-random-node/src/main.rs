// Definition for singly-linked list.

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
struct Solution {
    data: Vec<i32>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Solution {

    fn new(head: Option<Box<ListNode>>) -> Self {
        let mut data = Vec::new();
        let mut node = head;
        while let Some(n) = node {
            data.push(n.val);
            node = n.next;
        }
        Solution { data }
    }

    fn get_random(&self) -> i32 {
      let mut rng = rand::thread_rng();
      let idx = rand::Rng::gen_range(&mut rng, 0..self.data.len());
        self.data[idx]
    }
}

/**
 * Your Solution object will be instantiated and called as such:
 * let obj = Solution::new(head);
 * let ret_1: i32 = obj.get_random();
 */
fn main() {
    let mut head = Box::new(ListNode::new(1));
    head.next = Some(Box::new(ListNode::new(2)));
    if let Some(second) = head.next.as_mut() {
        second.next = Some(Box::new(ListNode::new(3)));
    }
    let obj = Solution::new(Some(head));

    for _ in 0..100 {
        let value = obj.get_random();
        assert!((1..=3).contains(&value));
    }
}
