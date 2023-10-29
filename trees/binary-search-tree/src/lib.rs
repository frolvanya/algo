#[derive(PartialEq, Clone, Debug)]
struct BinarySearchTree<T> {
    val: T,
    left: Node<T>,
    right: Node<T>,
}

type Node<T> = Option<Box<BinarySearchTree<T>>>;

impl<T: PartialOrd + Clone> BinarySearchTree<T> {
    fn new(val: T) -> Self {
        BinarySearchTree {
            val,
            left: None,
            right: None,
        }
    }

    fn insert(root: &mut Node<T>, val: T) {
        if let Some(inside_root) = root {
            if val < inside_root.val {
                match inside_root.left {
                    Some(_) => Self::insert(&mut inside_root.left, val),
                    None => inside_root.left = Some(Box::new(BinarySearchTree::new(val))),
                }
            } else {
                match inside_root.right {
                    Some(_) => Self::insert(&mut inside_root.right, val),
                    None => inside_root.right = Some(Box::new(BinarySearchTree::new(val))),
                }
            }
        }
    }

    fn delete(root: &mut Node<T>, val: T) {
        if let Some(ref mut inside_root) = root {
            if inside_root.val < val {
                if inside_root.right.is_some() {
                    Self::delete(&mut inside_root.right, val);
                }
            } else if inside_root.val > val {
                if inside_root.left.is_some() {
                    Self::delete(&mut inside_root.left, val);
                }
            } else if inside_root.left.is_none() {
                *root = inside_root.right.take();
            } else if inside_root.right.is_none() {
                *root = inside_root.left.take();
            } else {
                let mut right = inside_root.right.take().unwrap();
                let left = inside_root.left.take().unwrap();
                let mut node = &mut right;

                while let Some(ref mut next) = node.left {
                    node = next;
                }

                node.left = Some(left);
                *root = Some(right);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bst() {
        let mut bst = Some(Box::new(BinarySearchTree::new(5)));

        BinarySearchTree::insert(&mut bst, 3);
        BinarySearchTree::insert(&mut bst, 8);
        BinarySearchTree::insert(&mut bst, 4);
        BinarySearchTree::insert(&mut bst, 2);
        BinarySearchTree::insert(&mut bst, 9);

        assert_eq!(bst.clone().unwrap().val, 5);
        assert_eq!(bst.clone().unwrap().left.as_ref().unwrap().val, 3);
        assert_eq!(bst.clone().unwrap().right.as_ref().unwrap().val, 8);
        assert_eq!(
            bst.clone()
                .unwrap()
                .left
                .as_ref()
                .unwrap()
                .right
                .as_ref()
                .unwrap()
                .val,
            4
        );
        assert_eq!(
            bst.clone()
                .unwrap()
                .left
                .as_ref()
                .unwrap()
                .left
                .as_ref()
                .unwrap()
                .val,
            2
        );
        assert_eq!(
            bst.clone()
                .unwrap()
                .right
                .as_ref()
                .unwrap()
                .right
                .as_ref()
                .unwrap()
                .val,
            9
        );

        BinarySearchTree::delete(&mut bst, 3);
        assert_eq!(bst.clone().unwrap().left.as_ref().unwrap().val, 4);

        BinarySearchTree::delete(&mut bst, 5);
        assert_eq!(bst.clone().unwrap().val, 8);

        BinarySearchTree::delete(&mut bst, 9);
        assert_eq!(bst.clone().unwrap().right, None);
    }
}
