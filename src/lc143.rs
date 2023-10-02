type List = Option<Box<ListNode>>;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: List,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val,
        }
    }
}

pub fn reorder_list(head: &mut List) {
    let length = len(head);
    if length <= 2 {
        return; // nothing to do
    }
    // reverse list and patch original
    let mut clo = head.clone();
    walk(&mut clo, length / 2);
    reverse(&mut clo);
    let mut rev = clo;

    zip(head, &mut rev, length / 2);
}

fn zip(ord: &mut List, rev: &mut List, length: i32) {
    if rev.is_none() {
        ord.as_mut().unwrap().next.take();
        return;
    }
    if length == 0 && ord.as_ref().unwrap().val == rev.as_ref().unwrap().val {
        ord.as_mut().unwrap().next.take();
        return;
    }
    if rev.as_ref().unwrap().next.is_none() {
        ord.as_mut().unwrap().next.replace(rev.take().unwrap());
        return;
    }
    let n1 = ord.as_mut().unwrap();
    let mut n9 = rev.take();
    let n2 = n1.next.take();
    let mut n8 = n9.as_mut().unwrap().next.take();
    n1.next = n9.take();
    n1.next.as_mut().unwrap().next = n2;
    zip(&mut n1.next.as_mut().unwrap().next, &mut n8, length - 1)
}

fn walk(list: &mut List, n: i32) {
    if n == 0 {
        return;
    }
    for _ in 0..n {
        *list = list.take().unwrap().next;
    }
}

fn reverse(list: &mut List) {
    let mut node: List = list.take();
    while let Some(mut curr) = node {
        let next: List = curr.next.take();
        curr.next = list.take();
        *list = Some(curr);
        node = next;
    }
}

fn print(list: &List) {
    match list {
        None => println!("->🏁"),
        Some(node) => {
            print!("->{}", node.val);
            print(&node.next);
        }
    }
}


fn len(list: &List) -> i32 {
    match list {
        None => 0,
        Some(node) => 1 + len(&node.next),
    }
}

fn eq(l: &List, s: &[i32]) -> bool {
    if s.is_empty() {
        return l.is_none();
    }
    match l {
        None => s.len() == 0,
        Some(ln) => {
            ln.val.eq(&s[0]) && eq(&ln.next, &s[1..s.len()])
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reorder_list() {
        let mut list = Some::<Box<ListNode>>(Box::new(
            ListNode {
                val: 1,
                next: Some::<Box<ListNode>>(Box::new(
                    ListNode {
                        val: 2,
                        next: Some::<Box<ListNode>>(Box::new(
                            ListNode {
                                val: 3,
                                next: Some::<Box<ListNode>>(Box::new(
                                    ListNode {
                                        val: 4,
                                        next: Some::<Box<ListNode>>(Box::new(
                                            ListNode {
                                                val: 5,
                                                next: None,
                                            })),
                                    })),
                            })),
                    })),
            }));
        print!("lis: ");
        print(&list);
        assert!(eq(&list, &[1, 2, 3, 4, 5]));

        let mut clo = list.clone();
        print!("clo: ");
        print(&clo);
        assert!(eq(&clo, &[1, 2, 3, 4, 5]));

        walk(&mut clo, 2);
        let mut mid = clo;
        print!("mid: ");
        print(&mid);
        assert!(eq(&mid, &[3, 4, 5]));

        reverse(&mut mid);
        let rev = mid;
        print!("rev: ");
        print(&rev);
        assert!(eq(&rev, &[5, 4, 3]));

        reorder_list(&mut list);
        print!("reo: ");
        print(&list);
        assert!(eq(&list, &[1, 5, 2, 4, 3]));

        let mut list = Some::<Box<ListNode>>(Box::new(
            ListNode {
                val: 1,
                next: Some::<Box<ListNode>>(Box::new(
                    ListNode {
                        val: 2,
                        next: Some::<Box<ListNode>>(Box::new(
                            ListNode {
                                val: 3,
                                next: Some::<Box<ListNode>>(Box::new(
                                    ListNode {
                                        val: 4,
                                        next: None,
                                    })),
                            })),
                    })),
            }));
        reorder_list(&mut list);
        print!("reo: ");
        print(&list);
        assert!(eq(&list, &[1, 4, 2, 3]));
    }
}
