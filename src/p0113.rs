use crate::Solution;
use crate::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> Vec<Vec<i32>> {
        let mut paths = Vec::new();
        Self::find_paths(root, &mut Vec::new(), target_sum, &mut paths);
        paths
    }

    fn find_paths(
        root: Option<Rc<RefCell<TreeNode>>>,
        path: &mut Vec<i32>,
        mut sum: i32,
        paths: &mut Vec<Vec<i32>>,
    ) {
        if let Some(node) = root {
            let node = node.borrow();
            path.push(node.val);
            sum -= node.val;
            if node.left.is_none() && node.right.is_none() {
                if sum == 0 {
                    paths.push(path.clone());
                }
            } else {
                Self::find_paths(node.left.clone(), path, sum, paths);
                Self::find_paths(node.right.clone(), path, sum, paths);
            }
            path.pop();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(
            Solution::path_sum(
                TreeNode::from(vec![
                    Some(5),
                    Some(4),
                    Some(8),
                    Some(11),
                    None,
                    Some(13),
                    Some(4),
                    Some(7),
                    Some(2),
                    None,
                    None,
                    None,
                    None,
                    Some(5),
                    Some(1)
                ]),
                22
            ),
            [[5, 4, 11, 2], [5, 8, 4, 5]]
        );
        const EMPTY: Vec<Vec<i32>> = Vec::new();
        assert_eq!(
            Solution::path_sum(TreeNode::from(vec![Some(1), Some(2), Some(3)]), 5),
            EMPTY
        );
        assert_eq!(
            Solution::path_sum(TreeNode::from(vec![Some(1), Some(2)]), 0),
            EMPTY
        );
    }
}
