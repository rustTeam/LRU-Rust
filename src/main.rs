mod lib;

use std::cell::RefCell;
use std::rc::Rc;
use lib::double_list::ListNode;
use lib::double_list::List;

fn main() {
    let list : Rc<RefCell<ListNode>> = ListNode::new(50);

    println!("Hello, world!");
}
