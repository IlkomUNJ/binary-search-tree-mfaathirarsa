use std::cell::RefCell;
use std::rc::{Rc, Weak};

pub type BstNodeLink = Rc<RefCell<BstNode>>;
pub type WeakBstNodeLink = Weak<RefCell<BstNode>>;

//this package implement BST wrapper
#[derive(Debug, Clone)]
pub struct BstNode {
    pub key: Option<i32>,
    pub parent: Option<WeakBstNodeLink>,
    pub left: Option<BstNodeLink>,
    pub right: Option<BstNodeLink>,
}

impl BstNode {
    //private interface
    fn new(key: i32) -> Self {
        BstNode {
            key: Some(key),
            left: None,
            right: None,
            parent: None,
        }
    }

    pub fn new_bst_nodelink(value: i32) -> BstNodeLink {
        let currentnode = BstNode::new(value);
        let currentlink = Rc::new(RefCell::new(currentnode));
        currentlink
    }

    /**
     * Get a copy of node link
     */
    pub fn get_bst_nodelink_copy(&self) -> BstNodeLink {
        Rc::new(RefCell::new(self.clone()))
    }

    fn downgrade(node: &BstNodeLink) -> WeakBstNodeLink {
        Rc::<RefCell<BstNode>>::downgrade(node)
    }

    //private interface
    fn new_with_parent(parent: &BstNodeLink, value: i32) -> BstNodeLink {
        let mut currentnode = BstNode::new(value);
        //currentnode.add_parent(Rc::<RefCell<BstNode>>::downgrade(parent));
        currentnode.parent = Some(BstNode::downgrade(parent));
        let currentlink = Rc::new(RefCell::new(currentnode));
        currentlink
    }

    //add new left child, set the parent to current_node_link
    pub fn add_left_child(&mut self, current_node_link: &BstNodeLink, value: i32) {
        let new_node = BstNode::new_with_parent(current_node_link, value);
        self.left = Some(new_node);
    }

    //add new left child, set the parent to current_node_link
    pub fn add_right_child(&mut self, current_node_link: &BstNodeLink, value: i32) {
        let new_node = BstNode::new_with_parent(current_node_link, value);
        self.right = Some(new_node);
    }

    //search the current tree which node fit the value
    pub fn tree_search(&self, value: &i32) -> Option<BstNodeLink> {
        if let Some(key) = self.key {
            if key == *value {
                return Some(self.get_bst_nodelink_copy());
            }
            if *value < key && self.left.is_some() {
                return self.left.as_ref().unwrap().borrow().tree_search(value);
            } else if self.right.is_some() {
                return self.right.as_ref().unwrap().borrow().tree_search(value);
            }
        }
        //default if current node is NIL
        None
    }

    /**seek minimum by recurs
     * in BST minimum always on the left
     */
    pub fn minimum(&self) -> BstNodeLink {
        if self.key.is_some() {
            if let Some(left_node) = &self.left {
                return left_node.borrow().minimum();
            }
        }
        self.get_bst_nodelink_copy()
    }

    pub fn maximum(&self) -> BstNodeLink {
        if self.key.is_some() {
            if let Some(right_node) = &self.right {
                return right_node.borrow().maximum();
            }
        }
        self.get_bst_nodelink_copy()
    }

    /**
     * Return the root of a node, return self if not exist
     */
    pub fn get_root(node: &BstNodeLink) -> BstNodeLink {
        let parent = BstNode::upgrade_weak_to_strong(node.borrow().parent.clone());
        if parent.is_none() {
            return node.clone();
        }
        return BstNode::get_root(&parent.unwrap());
    }

    /**
     * NOTE: Buggy from pull request
     * Find node successor according to the book
     * Should return None, if x_node is the highest key in the tree
     */
    pub fn tree_successor(x_node: &BstNodeLink) -> Option<BstNodeLink> {
        // directly check if the node has a right child, otherwise go to the next block
        if let Some(right_node) = &x_node.borrow().right {
            return Some(right_node.borrow().minimum());
        } 
        
        // empty right child case
        else { 
            let mut x_node = x_node;
            let mut y_node = BstNode::upgrade_weak_to_strong(x_node.borrow().parent.clone());
            let mut temp: BstNodeLink;

            while let Some(ref exist) = y_node {
                if let Some(ref left_child) = exist.borrow().left {
                    if BstNode::is_node_match(left_child, x_node) {
                        return Some(exist.clone());
                    }
                }

                temp = y_node.unwrap();
                x_node = &temp;
                y_node = BstNode::upgrade_weak_to_strong(temp.borrow().parent.clone());
            }

            None    
        }
    }

    /**
     * Alternate simpler version of tree_successor that made use of is_nil checking
     */
    #[allow(dead_code)]
    pub fn tree_successor_simpler(x_node: &BstNodeLink) -> Option<BstNodeLink> {
        let x_ref = x_node.borrow();
    
        // Case 1: has right child → return its subtree's minimum
        if let Some(ref right) = x_ref.right {
            return Some(right.borrow().minimum());
        }
    
        // Case 2: no right child → go up until we move up from a left child
        let mut current = Rc::clone(x_node);
        let mut parent_opt = BstNode::upgrade_weak_to_strong(x_ref.parent.clone());
    
        drop(x_ref); // Drop borrow before the loop
    
        while let Some(parent_rc) = parent_opt {
            let is_left_child = {
                let parent_ref = parent_rc.borrow();
                parent_ref.left.as_ref().map_or(false, |left| Rc::ptr_eq(&current, left))
            };
    
            if is_left_child {
                return Some(Rc::clone(&parent_rc)); // ✅ Return clone, not the borrowed one
            }
    
            current = Rc::clone(&parent_rc);
            parent_opt = BstNode::upgrade_weak_to_strong(current.borrow().parent.clone());
        }
    
        None
    }
    
    /**
     * private function return true if node doesn't has parent nor children nor key
     */
    fn is_nil(node: &Option<BstNodeLink>) -> bool {
        match node {
            None => true,
            Some(x) => {
                if x.borrow().parent.is_none()
                    || x.borrow().left.is_none()
                    || x.borrow().right.is_none()
                {
                    return true;
                }
                return false;
            }
        }
    }

    //helper function to compare both nodelink
    fn is_node_match_option(node1: Option<BstNodeLink>, node2: Option<BstNodeLink>) -> bool {
        if node1.is_none() && node2.is_none() {
            return true;
        }
        if let Some(node1v) = node1 {
            return node2.is_some_and(|x: BstNodeLink| x.borrow().key == node1v.borrow().key);
        }
        return false;
    }

    fn is_node_match(anode: &BstNodeLink, bnode: &BstNodeLink) -> bool {
        if anode.borrow().key == bnode.borrow().key {
            return true;
        }
        return false;
    }

    /**
     * As the name implied, used to upgrade parent node to strong nodelink
     */
    fn upgrade_weak_to_strong(node: Option<WeakBstNodeLink>) -> Option<BstNodeLink> {
        match node {
            None => None,
            Some(x) => Some(x.upgrade().unwrap()),
        }
    }
    
    /// Insert a node into the BST
    pub fn tree_insert(root: &mut Option<BstNodeLink>, z: BstNodeLink) {
        let mut y: Option<BstNodeLink> = None;
        let mut x = root.clone();
    
        while x.is_some() {
            let current = x.unwrap();
            y = Some(current.clone());
    
            if z.borrow().key < current.borrow().key {
                x = current.borrow().left.clone();
            } else {
                x = current.borrow().right.clone();
            }
        }
    
        z.borrow_mut().parent = y.as_ref().map(|n| Rc::downgrade(n));
    
        if let Some(ref y_node) = y {
            if z.borrow().key < y_node.borrow().key {
                y_node.borrow_mut().left = Some(z.clone());
            } else {
                y_node.borrow_mut().right = Some(z.clone());
            }
        } else {
            *root = Some(z);
        }
    }

    /// Replaces one subtree as a child of its parent with another subtree
    pub fn transplant(root: &mut Option<BstNodeLink>, u: &BstNodeLink, v: Option<BstNodeLink>) {
        let parent = BstNode::upgrade_weak_to_strong(u.borrow().parent.clone());

        if let Some(ref parent_node) = parent {
            if parent_node.borrow().left.as_ref().map(|n| Rc::ptr_eq(n, u)).unwrap_or(false) {
                parent_node.borrow_mut().left = v.clone();
            } else {
                parent_node.borrow_mut().right = v.clone();
            }
        } else {
            *root = v.clone();
        }

        if let Some(ref v_node) = v {
            v_node.borrow_mut().parent = parent.map(|p| Rc::downgrade(&p));
        }
    }

    /// Delete a node from the BST
    pub fn tree_delete(root: &mut Option<BstNodeLink>, z: &BstNodeLink) {
        if z.borrow().left.is_none() {
            BstNode::transplant(root, z, z.borrow().right.clone());
        } else if z.borrow().right.is_none() {
            BstNode::transplant(root, z, z.borrow().left.clone());
        } else {
            let y = z.borrow().right.as_ref().unwrap().borrow().minimum();
    
            if !Rc::ptr_eq(&y, z.borrow().right.as_ref().unwrap()) {
                let y_right = y.borrow().right.clone();
                BstNode::transplant(root, &y, y_right);
    
                // We must drop the borrow before the next borrow
                {
                    let mut y_mut = y.borrow_mut();
                    y_mut.right = z.borrow().right.clone();
                    if let Some(ref right_node) = y_mut.right {
                        right_node.borrow_mut().parent = Some(Rc::downgrade(&y));
                    }
                }
            }
    
            BstNode::transplant(root, z, Some(y.clone()));
    
            {
                let mut y_mut = y.borrow_mut();
                y_mut.left = z.borrow().left.clone();
                if let Some(ref left_node) = y_mut.left {
                    left_node.borrow_mut().parent = Some(Rc::downgrade(&y));
                }
            }
        }
    }            
}
