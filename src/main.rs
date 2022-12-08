mod lib;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use lib::double_list::ListNode;
use lib::double_list::List;

fn new_rc(k: i32, v: i32)-> Option<Rc<RefCell<ListNode>>> {
    let node = ListNode::new(k, v);
    return Some(node);
}

fn test() {
    let mut mp = HashMap::new();
    let mut list = List::new();
    let node1 = new_rc(1, 1);
    let node2 = new_rc(2, 2);
    let node3 = new_rc(3, 3);
    mp.insert(1, node1.clone());
    //println!("{}", list.head.is_none());
    list.add_to_head(node1);
    //println!("{}", list.head.is_none());
    list.add_to_head(node2);
    list.add_to_head(node3);
    list.show_all();
    let x = 1;
    if let Some(t) = mp.get(&x) {
        list.move_to_head(t.clone());
    }
    list.show_all();
}

fn main() {
    test();
    println!("Hello, world!");
}
