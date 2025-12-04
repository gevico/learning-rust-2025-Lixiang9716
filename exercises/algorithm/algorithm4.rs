/*
	binary_search tree
	This problem requires you to implement a basic interface for a binary tree
*/

use std::cmp::Ordering;
use std::fmt::Debug;


#[derive(Debug)]
struct TreeNode<T>
where
    T: Ord,
{
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

#[derive(Debug)]
struct BinarySearchTree<T>
where
    T: Ord,
{
    root: Option<Box<TreeNode<T>>>,
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    fn new(value: T) -> Self {
        TreeNode {
            value,
            left: None,
            right: None,
        }
    }
}

impl<T> BinarySearchTree<T>
where
    T: Ord,
{

    fn new() -> Self {
        BinarySearchTree { root: None }
    }

    // Insert a value into the BST
    fn insert(&mut self, value: T) {
        let mut current = &mut self.root; // current 始终是可变引用

        loop {
            match current {
                // 当前节点为空，直接插入新节点
                None => {
                    *current = Some(Box::new(TreeNode::new(value)));
                    return;
                }
                // 当前节点存在，获取可变引用（关键：ref mut 绑定可变引用）
                Some(ref mut node) => {
                    match value.cmp(&node.value) {
                        // 新值等于当前节点值，不插入（BST 不允许重复）
                        Ordering::Equal => return,
                        // 新值小于当前节点值，向左子树遍历
                        Ordering::Less => {
                            current = &mut node.left; // 指向左子树的可变引用
                        }
                        // 新值大于当前节点值，向右子树遍历
                        Ordering::Greater => {
                            current = &mut node.right; // 指向右子树的可变引用
                        }
                    }
                }
            }
        } 

    }

    // Search for a value in the BST
    fn search(&self, value: T) -> bool {
        let mut current = &self.root;

        while let Some(node) = current {
            match value.cmp(&node.value) {
                Ordering::Equal => return true, // 找到值
                Ordering::Less => current = &node.left, // 向左子树找
                Ordering::Greater => current = &node.right, // 向右子树找
            }
        }

        false // 遍历完未找
    }
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    // Insert a node into the tree
    fn insert(&mut self, value: T) {
        //TODO
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_search() {
        let mut bst = BinarySearchTree::new();

        
        assert_eq!(bst.search(1), false);

        
        bst.insert(5);
        bst.insert(3);
        bst.insert(7);
        bst.insert(2);
        bst.insert(4);

        
        assert_eq!(bst.search(5), true);
        assert_eq!(bst.search(3), true);
        assert_eq!(bst.search(7), true);
        assert_eq!(bst.search(2), true);
        assert_eq!(bst.search(4), true);

        
        assert_eq!(bst.search(1), false);
        assert_eq!(bst.search(6), false);
    }

    #[test]
    fn test_insert_duplicate() {
        let mut bst = BinarySearchTree::new();

        
        bst.insert(1);
        bst.insert(1);

        
        assert_eq!(bst.search(1), true);

        
        match bst.root {
            Some(ref node) => {
                assert!(node.left.is_none());
                assert!(node.right.is_none());
            },
            None => panic!("Root should not be None after insertion"),
        }
    }
}    


