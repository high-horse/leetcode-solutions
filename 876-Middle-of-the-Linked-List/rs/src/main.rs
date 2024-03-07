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

pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut fast = &head;
    let mut slow = & head;
    
    while fast.as_ref().and_then(|node| node.next.as_ref()).is_some() {
        slow = &slow.as_ref().unwrap().next;
        fast = &fast.as_ref().unwrap().next.as_ref().unwrap().next;
    }
    
    slow.clone()
}


fn main() {
    // create linked list with these items
    let node5 = Box::new(ListNode::new(5));
    let node4 = Box::new(ListNode {
        val: 4,
        next: Some(node5),
    });
    let node3 = Box::new(ListNode {
        val: 3,
        next: Some(node4),
    });
    let node2 = Box::new(ListNode {
        val: 2,
        next: Some(node3),
    });
     let head = Box::new(ListNode {
        val: 1,
        next: Some(node2),
    });
    
    let mid_one = middle_node(Some(head));
    println!("{:?}", mid_one);
}
