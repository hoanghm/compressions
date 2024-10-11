use core::f64;
use std::cmp::{Eq, Ord, PartialEq, PartialOrd, Reverse};
use std::collections::BinaryHeap;

#[derive(Debug)]
struct Node {
    symbol: Option<char>,
    freq: f64,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
} 
impl Node {
    fn new(symbol: char, freq: f64) -> Node {
        Node {
            symbol: Some(symbol),
            freq,
            left: None,
            right: None,
        }
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        (self.freq - other.freq).abs() < f64::EPSILON
    }
}
impl Eq for Node {}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.freq.partial_cmp(&other.freq)
    }
}
impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.freq.total_cmp(&other.freq)
    }
}


fn printTree(pot_node: Option<Box<Node>>, encoding: Vec<u8>) {

    if let Some(node) = pot_node {
        if node.left.is_none() && node.right.is_none() {
            let encoding_str: String = encoding.iter().map(|c| c.to_string()).collect();
            println!("{:?}: {}", node.symbol, encoding_str);
            return;
        }

        let mut left_encoding = encoding.clone();
        left_encoding.push(0);
        let mut right_encoding = encoding.clone();
        right_encoding.push(1);

        printTree(node.left, left_encoding);
        printTree(node.right, right_encoding);
    }

}



fn main() {
    
    let chars = ['a','b','c','d','e','f'];
    let freq = [5., 9., 12., 13., 16., 45.];

    let mut nodes = BinaryHeap::new();

    for i in 0..chars.len() {
        let node = Node::new(chars[i], freq[i]);
        nodes.push(Reverse(node));
    }

    while nodes.len() > 1 {
        let left = nodes.pop().unwrap().0;
        let right = nodes.pop().unwrap().0;

        let new_node = Node {
            symbol: None,
            freq: left.freq + right.freq,

            left: Some(Box::new(left)),
            right: Some(Box::new(right)),
        };

        nodes.push(Reverse(new_node));
    }

    if let Some(Reverse(node)) = nodes.pop() {
        printTree(Some(Box::new(node)), vec![]);
    }

}
