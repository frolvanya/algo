#[derive(Debug, Clone)]
struct BinaryNode<T> {
    val: T,
    left: BinaryTree<T>,
    right: BinaryTree<T>,
}

type BinaryTree<T> = Option<Box<BinaryNode<T>>>;

impl<T: std::fmt::Display + std::fmt::Debug + Clone> BinaryNode<T> {
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
            Self::inorder_traversal(node.left, path);
            Self::inorder_traversal(node.right, path);
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
}

fn main() {
    let mut root = BinaryNode::new(7);

    root.left = Some(Box::new(BinaryNode::new(23)));
    root.right = Some(Box::new(BinaryNode::new(3)));

    root.left.as_mut().unwrap().left = Some(Box::new(BinaryNode::new(5)));
    root.left.as_mut().unwrap().right = Some(Box::new(BinaryNode::new(4)));

    root.right.as_mut().unwrap().left = Some(Box::new(BinaryNode::new(18)));
    root.right.as_mut().unwrap().right = Some(Box::new(BinaryNode::new(21)));

    let new_root = Some(Box::new(root.clone()));
    let mut path = Vec::new();

    BinaryNode::preorder_traversal(new_root.clone(), &mut path);
    println!("Preorder traversal: {:?}", path);
    path.clear();

    BinaryNode::inorder_traversal(new_root.clone(), &mut path);
    println!("Inorder traversal: {:?}", path);
    path.clear();

    BinaryNode::postorder_traversal(new_root, &mut path);
    println!("Postorder traversal: {:?}", path);
    path.clear();
}
