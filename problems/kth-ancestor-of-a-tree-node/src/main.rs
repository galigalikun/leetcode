struct TreeAncestor {
    n: i32,
    parent: Vec<i32>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl TreeAncestor {

    fn new(n: i32, parent: Vec<i32>) -> Self {
        TreeAncestor{
            n: n,
            parent: parent,
        }
    }
    
    fn get_kth_ancestor(&self, node: i32, k: i32) -> i32 {
        let mut node = node;
        let mut k = k;
        while k > 0 && node != -1 {
            let mut i = 0;
            while i < 32 && k >= (1 << i) {
                i += 1;
            }
            i -= 1;
            node = self.parent[node as usize];
            k -= 1 << i;
        }
        return node;
    }
}

/**
 * Your TreeAncestor object will be instantiated and called as such:
 * let obj = TreeAncestor::new(n, parent);
 * let ret_1: i32 = obj.get_kth_ancestor(node, k);
 */
fn main() {
    let obj = TreeAncestor::new(7, vec![-1, 0, 0, 1, 1, 2, 2]);
    assert_eq!(1, obj.get_kth_ancestor(3, 1));
    assert_eq!(0, obj.get_kth_ancestor(5, 2));
    assert_eq!(-1, obj.get_kth_ancestor(6, 3));
}
