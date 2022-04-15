use std::{cmp::Ordering::*, fs::File};
use std::io::{prelude, Read, BufRead};
use std::io::BufReader;

struct TreeNode<T: Ord + ToString> {
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

impl<T: Ord + ToString> TreeNode<T> {
    pub fn new(value: T) -> TreeNode<T> {
        TreeNode {
            value,
            left: None,
            right: None,
        }
    }

    pub fn insert(&mut self, new_value: T) {
        match new_value.cmp(&self.value) {
            Equal => panic!("Tree is not smart enough to handle equal values"),
            Less => {
                match self.left {
                    Some(ref mut node) => node.insert(new_value),
                    None => self.left = Some(Box::new(TreeNode::new(new_value))),
                }
            }
            Greater => {
                match self.right {
                    Some(ref mut node) => node.insert(new_value),
                    None => self.right = Some(Box::new(TreeNode::new(new_value))),
                }
            }
        }
    }
}

impl<T: Ord + ToString> ToString for TreeNode<T> {
    fn to_string(&self) -> String {
        let left_string = match &self.left {
            Some(node) => node.to_string() + ", ",
            None => String::new(),
        };
        let right_string = match &self.right {
            Some(node) => String::from(", ") + &node.to_string(),
            None => String::new(),
        };
        format!("{}{}{}", left_string, self.value.to_string(), right_string)
    }
}

fn main() {
    let mut root = TreeNode::new(5i32);

    let mut f = BufReader::new(File::open("numbers.txt").unwrap());

    let mut s = String::new();

    while f.read_line(&mut s).unwrap() != 0 {
        root.insert(s.trim().parse::<i32>().unwrap());
    }

    print!("{}", root.to_string())
}
