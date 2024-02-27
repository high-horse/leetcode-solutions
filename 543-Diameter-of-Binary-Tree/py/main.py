// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
// 
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut res: Vec<i32> = vec![0];   

        pub fn dfs(root: Option<Rc<RefCell<TreeNode>>>, res: &mut Vec<i32>) -> i32 {
            match root {
            None => -1,
            Some(n) => {
                let left = dfs(n.borrow().left.clone(), res);
                let right = dfs(n.borrow().right.clone(), res);

                let other = 2 + left + right;
                res[0] = res[0].max(other);

                1 + left.max(right)
            }
            }
        }         
        dfs(root, &mut res);
        res[0]
    }
}

    