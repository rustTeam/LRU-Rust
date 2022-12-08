pub mod double_list {
    use std::cell::RefCell;
    use std::collections::HashMap;
    use std::fmt::write;
    use std::rc::Rc;

    #[derive(Debug)]
    pub struct ListNode {
        key: i32,
        value: i32,
        next: Option<Rc<RefCell<ListNode>>>,
        prev: Option<Rc<RefCell<ListNode>>>
    }

    pub struct List {
        len: i32,
        pub head: Option<Rc<RefCell<ListNode>>>,
        pub tail: Option<Rc<RefCell<ListNode>>>
    }

    pub struct LRUCache {
        map: HashMap<i32, Option<Rc<RefCell<ListNode>>>>,
        list: List,
        size: i32,
        cap: i32,
    }

    impl ListNode {
        pub fn new(k: i32, v: i32) -> Rc<RefCell<ListNode>> {
            Rc::new(RefCell::new(ListNode{
                key: k,
                value: v,
                next: None,
                prev: None
            }))
        }
        pub fn display(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?} : {:?} =>", self.key, self.value)?;
            if let Some(node) = &self.next {
                return node.borrow().display(f);
            }
            write!(f,"")
        }
    }

    impl List {
        pub fn new() -> List {
            let head = ListNode::new(0, 0);
            let tail = ListNode::new(0, 0);
            head.borrow_mut().next = Some(Rc::clone(&tail));
            tail.borrow_mut().next = Some(Rc::clone(&head));
            return List {
                len: 0,
                head: Some(head),
                tail: Some(tail)
            }
        }

        pub fn add_to_head(&mut self, node: Option<Rc<RefCell<ListNode>>>) {
            if let Some(h) = self.head.take() {
                if let Some(n) = &node {
                    let hnxt = h.borrow_mut().next.take();
                    n.borrow_mut().prev = self.head.clone();
                    n.borrow_mut().next = hnxt.clone();
                    if let Some(hn) = &hnxt {
                        hn.borrow_mut().prev = node.clone();
                    }
                    h.borrow_mut().next = node.clone();
                }
                self.head = Some(h);
                self.len += 1;
            }
        }

        pub fn remove_node(&mut self, node: Option<Rc<RefCell<ListNode>>>) {
            if let Some(n) = &node {
                let pre = n.borrow_mut().prev.take();
                let nxt = n.borrow_mut().next.take();
                if let Some(p) = &pre {
                    p.borrow_mut().next = nxt.clone();
                }
                if let Some(nx) = &nxt {
                    nx.borrow_mut().prev = pre.clone();
                }
                // n.borrow_mut().prev = pre.clone();
                // n.borrow_mut().next = nxt.clone();
                self.len -= 1;
            }
        }

        pub fn move_to_head(&mut self, node: Option<Rc<RefCell<ListNode>>>) {
            let ncl = node.clone();
            self.remove_node(node);
            self.add_to_head(ncl);
        }

        pub fn remove_tail(&mut self) -> i32{
            let mut ans = 0;
            if let Some(t) = self.tail.take() {
                let node = t.borrow_mut().prev.take();
                if let Some(n) = &node {
                    ans = n.borrow_mut().key;
                }
                self.tail = Some(t);
                self.remove_node(node);
            }
            return ans
        }

        pub fn show_all(&mut self) {
            if let Some(n) = self.head.take() {
                let mut node = n.clone();
                for i in 1..self.len+2 {
                    if i != 1 && i != self.len + 2{
                        print!("key: {:?}, ", node.borrow().key);
                        println!("value: {:?}", node.borrow().value);
                    }
                    let tmp = node.borrow().next.as_ref().unwrap().clone();
                    node = tmp;
                }
                self.head = Some(n)
            }
        }

    }

    impl LRUCache {

        //TODO
        pub fn get(key: i32) -> i32{
            return 0;
        }

        //TODO
        pub fn put(key: i32, val: i32) -> i32 {
            return 0;
        }
    }

}