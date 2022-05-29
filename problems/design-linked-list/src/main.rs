#[derive(PartialEq, Eq, Clone, Debug)]
struct MyLinkedList {
    head: Option<Box<Node>>,
}

#[derive(PartialEq, Eq, Clone, Debug)]
struct Node {
    next: Option<Box<Node>>,
    val: i32,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyLinkedList {
    fn new() -> Self {
        MyLinkedList { head: None }
    }

    fn get(&self, index: i32) -> i32 {
        let mut node = &self.head;
        let mut i = 0;
        while i < index {
            node = &node.as_ref().unwrap().next;
            i += 1;
        }
        return node
            .as_ref()
            .unwrap_or(&Box::new(Node {
                next: None,
                val: -1,
            }))
            .val;
    }

    fn add_at_head(&mut self, val: i32) {
        let node = Box::new(Node {
            next: self.head.take(),
            val: val,
        });
        self.head = Some(node);
    }

    fn add_at_tail(&mut self, val: i32) {
        let node = Box::new(Node {
            next: None,
            val: val,
        });
        if let Some(ref mut head) = self.head {
            let mut tail = head;
            while let Some(ref mut next) = tail.next {
                tail = next;
            }
            tail.next = Some(node);
        } else {
            self.head = Some(node);
        }
    }

    fn add_at_index(&mut self, index: i32, val: i32) {
        if index == 0 {
            self.add_at_head(val);
        } else {
            let mut node = Box::new(Node {
                next: None,
                val: val,
            });
            let mut head = &self.head;
            let mut i = 0;
            while i < index - 1 {
                head = &head.as_ref().unwrap().next;
                i += 1;
            }
            node.next = head.as_ref().unwrap().clone().next.take();
            self.head.as_mut().unwrap().next = Some(node);
        }
    }

    fn delete_at_index(&mut self, index: i32) {
        if index == 0 {
            self.head = self.head.as_ref().unwrap().clone().next.take();
        } else {
            let mut head = &self.head;
            let mut i = 0;
            while i < index - 1 {
                head = &head.as_ref().unwrap().next;
                i += 1;
            }
            if let Some(ref mut next) = head.as_ref().unwrap().clone().next {
                self.head.as_mut().unwrap().next = next.next.take();
            }
        }
    }
}

/**
 * Your MyLinkedList object will be instantiated and called as such:
 * let obj = MyLinkedList::new();
 * let ret_1: i32 = obj.get(index);
 * obj.add_at_head(val);
 * obj.add_at_tail(val);
 * obj.add_at_index(index, val);
 * obj.delete_at_index(index);
 */
fn main() {
    // let mut obj = MyLinkedList::new();
    // obj.add_at_head(1);
    // obj.add_at_tail(3);
    // obj.add_at_index(1, 2);
    // println!("debug {:?}", obj);
    // assert_eq!(obj.get(1), 2);
    // obj.delete_at_index(1);
    // println!("debug {:?}", obj);
    // assert_eq!(obj.get(1), 3);

    let mut obj = MyLinkedList::new();
    obj.add_at_head(7);
    obj.add_at_head(2);
    obj.add_at_head(1);
    obj.add_at_index(3, 0);
    println!("debug {:?}", obj);
    obj.delete_at_index(2);
    obj.add_at_head(6);
    obj.add_at_tail(4);
    println!("debug {:?}", obj);
    assert_eq!(obj.get(4), 4);
    obj.add_at_head(4);
    obj.add_at_index(5, 0);
    obj.add_at_head(6);
    println!("debug {:?}", obj);
}
