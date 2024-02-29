fn main() {
    println!("Hello, world!");
}

// Definition for a binary tree node.
 #[derive(Debug, PartialEq, Eq)]
 pub struct TreeNode {
   pub val: i32,
   pub left: Option<Rc<RefCell<TreeNode>>>,
   pub right: Option<Rc<RefCell<TreeNode>>>,
 }
 
 impl TreeNode {
   #[inline]
   pub fn new(val: i32) -> Self {
     TreeNode {
       val,
       left: None,
       right: None
     }
   }
 }
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;
impl Solution {
    pub fn is_even_odd_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let mut even = true;
        let mut q = VecDeque::new();
        q.push_back(root.unwrap());

        while ! q.is_empty() {
            let mut prev = if even {i32::MIN} else {i32::MAX};
            for _ in 0..q.len() {
                let node = q.pop_front().unwrap();
                let node = node.borrow();

                if even && (node.val % 2 == 0 || node.val <= prev) {
                    return false;
                } else if !even && (node.val % 2 == 1 || node.val >= prev) {
                    return false;
                }

                if let Some(left) = &node.left  {
                    q.push_back(Rc::clone(left));
                }
                if let Some(right) = &node.right {
                    q.push_back(Rc::clone(right));
                }

                prev = node.val;
            }
            even = !even;
        }
        true
    }
}
