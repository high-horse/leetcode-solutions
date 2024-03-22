
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

fn main() {
    let first = ListNode::new(1);
    
    
    let sec = ListNode{
        val: 2,
		next: Some(Box::new(first)),
    };
	
	let third = ListNode{
        val: 2,
		next: Some(Box::new(sec)),
    };
	let head = ListNode{
        val: 1,
		next: Some(Box::new(third)),
    };
	
	let answer = is_palindrome(Some(Box::new(head)));
	println!("{:?}", answer);
}




pub fn is_palindrome(mut head: Option<Box<ListNode>>) -> bool {

    let mut head_vec: Vec<i32> = vec![] ;
    
    while let Some(node) = head {
        head_vec.push(node.val);
        head = node.next;
    }
    
    let mut first = 0;
    let mut length = head_vec.len();
    // let length = head_vec.len();
    let mut last = length-1;
    
    let mut i = 0;
    let loop_end = length/2;
    while i < loop_end {
        if head_vec[first] != head_vec[last]{
            return false;
        }
        first += 1;
        last -= 1;
        i += 1;
    }
    true
}
