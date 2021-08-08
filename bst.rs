use std::io;
use std::cmp::max;
use std::collections::VecDeque;
use std::borrow::Borrow;

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


        let mut queue: VecDeque<Box<BST>> = VecDeque::new();
        queue.push_back(node.unwrap_or(BST::bst_on_heap(0)));


        while queue.borrow().len() != 0 {

            let queue_size = queue.len();

            for _ in 0..queue_size {

                let head = queue.front_mut().unwrap();

                print!("{} ", head.value);

                match head.left_node {
                    Some(mut left_node) => queue.push_back(left_node),
                    _ => continue,
                }

                match head.right_node {
                    Some(mut right_node) => queue.push_back(right_node),
                    _ => continue,
                }

            }
            println!();
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

    BST::level_order_traversal_map(root);


}
