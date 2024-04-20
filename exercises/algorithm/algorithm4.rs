/*
	binary_search tree
	This problem requires you to implement a basic interface for a binary tree
*/


use std::cmp::Ordering;
use std::fmt::Debug;


#[derive(Debug)]
#[derive(Clone)]
struct TreeNode<T>
where
    T: Ord+Clone,
{
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

#[derive(Debug)]
struct BinarySearchTree<T>
where
    T: Ord+Clone,
{
    root: Option<Box<TreeNode<T>>>,
}
// impl <T>Option<T>{
//     fn clone(&self)->Self{Self}
// }
impl<T> TreeNode<T>
where
    T: Ord+Clone,
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
    T: Ord+Clone,
{

    fn new() -> Self {
        BinarySearchTree { root: None }
    }

    // Insert a value into the BST
    fn insert(&mut self, value: T) {
        let r=self.root.clone();
        let r1=&mut self.root;
        match r{
            None=>{*r1=Some(Box::new(TreeNode::new(value.clone())));},
            _=>{}
        }
        
        Self::ins(r1,value);
        //TODO
    }
    
    fn ins(n:&mut Option<Box<TreeNode<T>>>,value:T)->bool{
        match n{
            None=>{return false;},
            Some(x)=>{
                let v=x.value.clone();
                let V=value.clone();
                let u:&mut Box<TreeNode<T>>=x;
                if V>v{
                    if !Self::ins(&mut u.right,V){u.right=Some(Box::new(TreeNode::new(value)));}
                }
                else if V<v{if !Self::ins(&mut u.left,V){u.left=Some(Box::new(TreeNode::new(value)));}}
                else{}
            }
            
        }
        true
    }
    // Search for a value in the BST
    fn search(&self, value: T) -> bool {
        //TODO
        let r=self.root.clone();
        let r1=self.root.clone();
        match r{
            None=>{return false;},
            _=>{}
        }
        
        Self::sear(r1,value)
    }
    fn sear(n:Option<Box<TreeNode<T>>>,value:T)->bool{
        match n{
            None=>{return false;},
            Some(ref x)=>{
                let v=&x.value;
                let V=value.clone();
                let mut u=n.clone().unwrap();
                if V==*v{return true;}
                if V>*v{if Self::sear(u.right,V){return true}}
                else {if Self::sear(u.left,V){return true}}
            }
            
        }
        false
    }
}

impl<T> TreeNode<T>
where
    T: Ord+Clone,
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


