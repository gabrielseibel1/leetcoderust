// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    #[allow(dead_code)]
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val,
        }
    }
}

pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut curr: Option<Box<ListNode>> = None;
    let mut next: Option<Box<ListNode>> = head;
    loop {
        if next.is_none() {
            break;
        }
        let mut next_node: Box<ListNode> = next.take().unwrap();
        let fwd = next_node.next.take();
        next_node.next = curr.take();
        curr = Some(next_node);
        next = fwd;
    }
    curr
}