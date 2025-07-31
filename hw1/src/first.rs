use std::mem;

#[derive(Debug)]
pub struct BST {
    root: Link,
}
#[derive(Debug)]
pub enum Link {
    Empty,
    More(Box<Node>),
}
#[derive(Debug)]
pub struct Node {
    elem: i32,
    left: Link,
    right: Link,
}

impl BST {
    pub fn new() -> Self {
        Self { root: Link::Empty }
    }

    pub fn insert(&mut self, elem: i32) -> bool {
        self.root.insert(elem)
    }
    pub fn search(&mut self, elem: i32) -> bool {
        self.root.search(elem)
    }
}
impl Link {
    pub fn insert(&mut self, elem: i32) -> bool {
        match self {
            Link::Empty => {
                let node = Box::new(Node {
                    elem: elem,
                    left: Link::Empty,
                    right: Link::Empty,
                });
                mem::replace(self, Link::More(node));
                return true;
            }
            Link::More(boxed_node) => {
                let node = boxed_node;
                if elem < node.elem {
                    node.right.insert(elem)
                } else if elem > node.elem {
                    node.left.insert(elem)
                } else {
                    false
                }
            }
        }
    }
    pub fn search(&mut self, elem: i32) -> bool {
        match self {
            Link::Empty => {
                // you just can't
                return false;
            }
            Link::More(boxed_node) => {
                let node = boxed_node;
                if elem < node.elem {
                    node.right.insert(elem)
                } else if elem > node.elem {
                    node.left.insert(elem)
                } else {
                    true
                }
            }
        }
    }
}

mod test {
    use crate::first::BST;

    #[test]
    fn just() {
        let mut tree = BST::new();
        // Populate list
        assert_eq!(tree.insert(1), true);
        assert_eq!(tree.insert(2), true);
        assert_eq!(tree.insert(3), true);

        // Check repeatly insertion
        assert_eq!(tree.insert(1), false)
    }
    #[test]
    fn search() {
        let mut tree = BST::new();
        assert_eq!(tree.insert(1), true);
        assert_eq!(tree.search(1), true)
    }
}
