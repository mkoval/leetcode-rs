// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
// 
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    fn add_two_numbers_recurse(l1: &Option<Box<ListNode>>,
                               l2: &Option<Box<ListNode>>,
                               carry: i32)
        -> Option<Box<ListNode>>
    {
        match (l1, l2) {
            (Some(n1), Some(n2)) => {
                let digit = n1.val + n2.val + carry;
                Some(Box::new(ListNode{
                    val: digit % 10,
                    next: Self::add_two_numbers_recurse(
                        &n1.next, &n2.next, digit / 10),
                }))
            }
            (Some(n1), None) => Some(n1.clone()),
            (None, Some(n2)) => Some(n2.clone()),
            (None, None) => {
                if carry != 0 {
                    Some(Box::new(ListNode::new(carry)))
                } else {
                    None
                }
            },
        }
    }
    
    pub fn add_two_numbers(l1: Option<Box<ListNode>>,
                           l2: Option<Box<ListNode>>)
        -> Option<Box<ListNode>>
    {
        Self::add_two_numbers_recurse(&l1, &l2, 0)
    }
}
