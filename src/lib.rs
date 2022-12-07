pub mod double_list {
    use std::cell::RefCell;
    use std::rc::Rc;

    #[derive(Debug)]
    pub struct ListNode {
        value: i32,
        next: Option<Rc<RefCell<ListNode>>>,
        prev: Option<Rc<RefCell<ListNode>>>
    }

    pub struct List {
        len: usize,
        head: Option<Rc<RefCell<ListNode>>>,
        tail: Option<Rc<RefCell<ListNode>>>
    }

    impl ListNode {
        pub fn new(value: i32) -> Rc<RefCell<ListNode>> {
            let tmp = Rc::new(RefCell::new(ListNode{
                value,
                next: None,
                prev: None
            }));
            return Rc::clone(&tmp);
        }
    }

    impl List {
        pub fn new() -> List {
            let first = ListNode::new(0);
            let last = ListNode::new(0);
            first.borrow().next = Some(Rc::clone(&last));
            last.borrow().next = Some(Rc::clone(&first));
            return List {
                len: 0,
                head: Some(first),
                tail: Some(last)
            }
        }

        pub fn insert(&mut self, index: i32, value: i32) {

        }
    }
}