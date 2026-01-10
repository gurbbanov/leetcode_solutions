use std::cell::RefCell;
use std::rc::Rc;

fn main() {}

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
            right: None,
        }
    }
}

struct Solution;

impl Solution {
    pub fn subtree_with_all_deepest(
        root: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        Self::find_deep(root).1
    }

    pub fn find_deep(
        branch: Option<Rc<RefCell<TreeNode>>>,
    ) -> (i32, Option<Rc<RefCell<TreeNode>>>) {
        match branch {
            None => (0, None),
            Some(br) => {
                let left = br.borrow().left.clone();
                let right = br.borrow().right.clone();

                let (left_depth, l_vector) = Self::find_deep(left);
                let (right_depth, r_vector) = Self::find_deep(right);

                if left_depth > right_depth {
                    return (left_depth + 1, l_vector);
                } else if right_depth > left_depth {
                    return (right_depth + 1, r_vector);
                } else {
                    return (left_depth + 1, Some(br));
                }
            }
        }
    }
}
