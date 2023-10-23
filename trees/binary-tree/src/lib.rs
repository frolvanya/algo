#[derive(Debug, Clone)]
struct BinaryNode<T> {
    val: T,
    left: BinaryTree<T>,
    right: BinaryTree<T>,
}

type BinaryTree<T> = Option<Box<BinaryNode<T>>>;

impl<T: std::fmt::Debug + PartialEq + Copy> BinaryNode<T> {
    fn new(val: T) -> Self {
        BinaryNode {
            val,
            left: None,
            right: None,
        }
    }

    fn preorder_traversal(root: BinaryTree<T>, path: &mut Vec<T>) {
        if let Some(node) = root.clone() {
            path.push(node.val);
            Self::preorder_traversal(node.left, path);
            Self::preorder_traversal(node.right, path);
        }
    }

    fn inorder_traversal(root: BinaryTree<T>, path: &mut Vec<T>) {
        if let Some(node) = root {
            Self::inorder_traversal(node.left, path);
            path.push(node.val);
            Self::inorder_traversal(node.right, path);
        }
    }

    fn postorder_traversal(root: BinaryTree<T>, path: &mut Vec<T>) {
        if let Some(node) = root {
            Self::postorder_traversal(node.left, path);
            Self::postorder_traversal(node.right, path);
            path.push(node.val);
        }
    }

    fn dfs(root: BinaryTree<T>, val: T) -> bool {
        if let Some(node) = root {
            if node.val == val {
                return true;
            }

            return Self::dfs(node.left, val) || Self::dfs(node.right, val);
        }

        false
    }

    fn bfs(root: BinaryNode<T>, val: T) -> bool {
        let mut q = std::collections::VecDeque::new();

        q.push_back(Box::new(root));

        while !q.is_empty() {
            let curr = q.pop_front();

            if let Some(inside_curr) = curr {
                if inside_curr.val == val {
                    return true;
                }

                if let Some(left_curr) = inside_curr.left {
                    q.push_back(left_curr);
                }
                if let Some(right_curr) = inside_curr.right {
                    q.push_back(right_curr);
                }
            }
        }

        false
    }

    fn compare(root1: BinaryTree<T>, root2: BinaryTree<T>) -> bool {
        if root1.is_none() && root2.is_none() {
            return true;
        }

        if root1.is_none() || root2.is_none() {
            return false;
        }

        if root1.clone().unwrap().val != root2.clone().unwrap().val {
            return false;
        }

        Self::compare(root1.clone().unwrap().left, root2.clone().unwrap().left)
            && Self::compare(root1.unwrap().right, root2.unwrap().right)
    }
}

#[cfg(test)]
pub mod tests {
    use super::BinaryNode;

    #[test]
    fn test_preorder_traversal() {
        let mut root = BinaryNode::new(7);

        root.left = Some(Box::new(BinaryNode::new(23)));
        root.right = Some(Box::new(BinaryNode::new(3)));

        root.left.as_mut().unwrap().left = Some(Box::new(BinaryNode::new(5)));
        root.left.as_mut().unwrap().right = Some(Box::new(BinaryNode::new(4)));

        root.right.as_mut().unwrap().left = Some(Box::new(BinaryNode::new(18)));
        root.right.as_mut().unwrap().right = Some(Box::new(BinaryNode::new(21)));

        let mut path = Vec::new();
        BinaryNode::preorder_traversal(Some(Box::new(root)), &mut path);
        dbg!(&path);
        assert_eq!(path, vec![7, 23, 5, 4, 3, 18, 21]);
    }

    #[test]
    fn test_inorder_traversal() {
        let mut root = BinaryNode::new(7);

        root.left = Some(Box::new(BinaryNode::new(23)));
        root.right = Some(Box::new(BinaryNode::new(3)));

        root.left.as_mut().unwrap().left = Some(Box::new(BinaryNode::new(5)));
        root.left.as_mut().unwrap().right = Some(Box::new(BinaryNode::new(4)));

        root.right.as_mut().unwrap().left = Some(Box::new(BinaryNode::new(18)));
        root.right.as_mut().unwrap().right = Some(Box::new(BinaryNode::new(21)));

        let mut path = Vec::new();
        BinaryNode::inorder_traversal(Some(Box::new(root)), &mut path);
        assert_eq!(path, vec![5, 23, 4, 7, 18, 3, 21]);
    }

    #[test]
    fn test_postorder_traversal() {
        let mut root = BinaryNode::new(7);

        root.left = Some(Box::new(BinaryNode::new(23)));
        root.right = Some(Box::new(BinaryNode::new(3)));

        root.left.as_mut().unwrap().left = Some(Box::new(BinaryNode::new(5)));
        root.left.as_mut().unwrap().right = Some(Box::new(BinaryNode::new(4)));

        root.right.as_mut().unwrap().left = Some(Box::new(BinaryNode::new(18)));
        root.right.as_mut().unwrap().right = Some(Box::new(BinaryNode::new(21)));

        let mut path = Vec::new();
        BinaryNode::postorder_traversal(Some(Box::new(root)), &mut path);
        assert_eq!(path, vec![5, 4, 23, 18, 21, 3, 7]);
    }

    #[test]
    fn test_compare() {
        let mut root1 = BinaryNode::new(7);

        root1.left = Some(Box::new(BinaryNode::new(23)));
        root1.right = Some(Box::new(BinaryNode::new(3)));

        root1.left.as_mut().unwrap().left = Some(Box::new(BinaryNode::new(5)));
        root1.left.as_mut().unwrap().right = Some(Box::new(BinaryNode::new(4)));

        root1.right.as_mut().unwrap().left = Some(Box::new(BinaryNode::new(18)));
        root1.right.as_mut().unwrap().right = Some(Box::new(BinaryNode::new(21)));

        let mut root2 = BinaryNode::new(7);

        root2.left = Some(Box::new(BinaryNode::new(23)));
        root2.right = Some(Box::new(BinaryNode::new(3)));

        root2.left.as_mut().unwrap().left = Some(Box::new(BinaryNode::new(5)));
        root2.left.as_mut().unwrap().right = Some(Box::new(BinaryNode::new(4)));

        root2.right.as_mut().unwrap().left = Some(Box::new(BinaryNode::new(18)));
        root2.right.as_mut().unwrap().right = Some(Box::new(BinaryNode::new(21)));

        let mut root3 = BinaryNode::new(7);

        root3.left = Some(Box::new(BinaryNode::new(23)));
        root3.right = Some(Box::new(BinaryNode::new(3)));

        assert!(BinaryNode::compare(
            Some(Box::new(root1.clone())),
            Some(Box::new(root2))
        ));

        assert!(!BinaryNode::compare(
            Some(Box::new(root1)),
            Some(Box::new(root3))
        ));
    }

    #[test]
    fn test_dfs() {
        let mut root = BinaryNode::new(7);

        root.left = Some(Box::new(BinaryNode::new(23)));
        root.right = Some(Box::new(BinaryNode::new(3)));

        root.left.as_mut().unwrap().left = Some(Box::new(BinaryNode::new(5)));
        root.left.as_mut().unwrap().right = Some(Box::new(BinaryNode::new(4)));

        root.right.as_mut().unwrap().left = Some(Box::new(BinaryNode::new(18)));
        root.right.as_mut().unwrap().right = Some(Box::new(BinaryNode::new(21)));

        assert!(BinaryNode::dfs(Some(Box::new(root.clone())), 5));
        assert!(!BinaryNode::dfs(Some(Box::new(root)), 10));
    }

    #[test]
    fn test_bfs() {
        let mut root = BinaryNode::new(7);

        root.left = Some(Box::new(BinaryNode::new(23)));
        root.right = Some(Box::new(BinaryNode::new(3)));

        root.left.as_mut().unwrap().left = Some(Box::new(BinaryNode::new(5)));
        root.left.as_mut().unwrap().right = Some(Box::new(BinaryNode::new(4)));

        root.right.as_mut().unwrap().left = Some(Box::new(BinaryNode::new(18)));
        root.right.as_mut().unwrap().right = Some(Box::new(BinaryNode::new(21)));

        assert!(BinaryNode::bfs(root.clone(), 5));
        assert!(!BinaryNode::bfs(root, 10));
    }
}
