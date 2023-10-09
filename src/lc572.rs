use std::rc::Rc;
use std::cell::RefCell;

type Tree = Option<Rc<RefCell<TreeNode>>>;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Tree,
    pub right: Tree,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

#[allow(dead_code)]
struct Solution {}

#[allow(dead_code)]
impl Solution {
    pub fn is_subtree(root: Tree, sub_root: Tree) -> bool {
        dfs_fits(&root, &sub_root)
    }
}


#[allow(dead_code)]
fn same(t1: &Tree, t2: &Tree) -> bool {
    if t2.is_none() || t1.is_none() {
        return t1 == t2;
    }
    let t1 = t1.as_ref().unwrap().borrow();
    let t2 = t2.as_ref().unwrap().borrow();
    t1.val == t2.val &&
        same(&t1.left, &t2.left) &&
        same(&t1.right, &t2.right)
}

#[allow(dead_code)]
fn dfs_fits(tree: &Tree, shape: &Tree) -> bool {
    match &tree {
        None => shape.is_none(),
        Some(t) => {
            if same(tree, shape) {
                return true;
            }
            dfs_fits(&t.borrow().left, shape) ||
                dfs_fits(&t.borrow().right, shape)
        }
    }
}