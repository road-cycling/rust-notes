use std::io;
use std::cmp::max;
use std::collections::VecDeque;
use std::collections::HashMap;
use std::borrow::{Borrow, BorrowMut};
use std::ops::Index;

#[derive(Debug)]
struct BST {
    value: u32,
    left_node: Option<Box<BST>>,
    right_node: Option<Box<BST>>,
}

impl BST {

    fn bst_on_heap(value: u32) -> Box<BST> {
        Box::new(
            BST {
                value,
                left_node: None,
                right_node: None
            }
        )
    }

    fn insert(node: Option<Box<BST>>, value: u32) -> Option<Box<BST>> {

        match node {
            Some(mut tree) => {
                if tree.value > value {
                    tree.right_node = BST::insert(tree.right_node, value)
                } else {
                    tree.left_node = BST::insert(tree.left_node, value)
                }
                Some(tree)
            },
            None => Some(BST::bst_on_heap(value))
        }
    }

    fn find(node: Option<Box<BST>>, target_value: u32) -> bool {

        match node {
            Some(tree) => {
                if tree.value > target_value {
                    BST::find(tree.left_node, target_value)
                } else if tree.value < target_value {
                    BST::find(tree.right_node, target_value)
                } else {
                    true
                }
            },
            None => false
        }
    }

    fn height(node: Option<Box<BST>>) -> i32 {
        match node {
            Some(tree) => {
                let lst = BST::height(tree.left_node);
                let rst = BST::height(tree.right_node);

                return max(lst, rst) + 1;
            },
            None => 0
        }
    }

    fn level_order_traversal_map(node: Option<Box<BST>>) {

        let mut levels: HashMap<i32, Vec<u32>> = HashMap::new();

        fn level_order_traversal(level_map: &mut HashMap<i32, Vec<u32>>, node: Option<Box<BST>>, level: i32) {

            match node {
                Some(tree) => {

                    let entry = level_map
                        .entry(level)
                        .or_insert(Vec::new());
                    entry.push(tree.value);

                    level_order_traversal(level_map, tree.left_node, level + 1);
                    level_order_traversal(level_map, tree.right_node, level + 1);
                },
                None => {}
            }
        }

        level_order_traversal(&mut levels, node, 1);

        for level in levels.borrow().keys() {
            println!("Level: {} - {:?}", level, levels.index(level))
        }

    }

    fn level_order_traversal_queue(node: Option<Box<BST>>) {

        let mut queue: VecDeque<Box<BST>> = VecDeque::new();
        queue.push_back(node.unwrap_or(BST::bst_on_heap(0)));


        while queue.len() != 0 {
            let queue_size = queue.len();

            for _ in 0..queue_size {

                let head = queue.pop_front().unwrap();
                print!("{} ", head.value);

                match head.left_node {
                    Some(left_node) => queue.push_back(left_node),
                    None => {},
                }

                match head.right_node {
                    Some(right_node) => queue.push_back(right_node),
                    None => {},
                }

            }
            println!();
        }
    }

    fn nth_node_in_tree(node: Option<Box<BST>>, nth: &mut i32) -> Option<u32> {
        match node {
            Some(n) => {
                match BST::nth_node_in_tree(n.left_node, nth) {
                    Some(v) => Some(v),
                    None => {
                        if *nth == 0 {
                            return Some(n.value)
                        }
                        *nth -= 1;

                        BST::nth_node_in_tree(n.right_node, nth)
                    },
                }
            },
            None => None
        }
    }
}

pub(crate) fn create_bst() {

    println!("Lets build a tree - an alphanumeric character will cause tree building to stop");

    let mut root = Some(BST::bst_on_heap(100));

    loop {

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input.");

        let input: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => break
        };


        root = BST::insert(root, input);
        println!("{:?}", root);


    }

    // Issue with borrow here -- need to reread docs.
    // BST::level_order_traversal_queue(root);
    BST::level_order_traversal_map(root);

}
