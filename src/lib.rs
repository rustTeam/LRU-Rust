pub mod double_list {
    use std::cell::RefCell;
    use std::collections::HashMap;
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
        map: HashMap<i32, Rc<RefCell<ListNode>>>,
        pub list: List,
        cap: i32,
    }
    pub struct HistoryList {
        list: List,
        node_map: HashMap<i32,Rc<RefCell<ListNode>>>,
        cnt_map: HashMap<i32, i32>,
        cap: i32,
    }
    pub struct LRUKCache {
        k: i32,
        hlist: HistoryList,
        lrulist: LRUCache,
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
            return ans;
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
        pub fn new(capacity: i32)  -> LRUCache{
            let list = List::new();
            let  map = HashMap::new();
            return LRUCache {
                list: list,
                map: map,
                cap: capacity
            }
        }
        //TODO
        pub fn get(&mut self, key: i32) -> i32{
            if let Some(node) = self.map.get(&key) {
                let val = node.borrow().value;
                self.list.move_to_head(Some(node.clone()));
                return val;
            } else {
                return -1;
            }
        }

        //TODO
        pub fn put(&mut self, key: i32, val: i32)  {
            if let Some(node) = self.map.get_mut(&key) {
                node.borrow_mut().value = val;
                self.list.move_to_head(Some(node.clone()));
            } else {
                if self.list.len == self.cap {
                    let key = self.list.remove_tail();
                    self.map.remove(&key);
                }
                let new_node = ListNode::new(key, val);
                self.map.insert(key, new_node.clone());
                self.list.add_to_head(Some(new_node));
            }

        }


    }

    impl HistoryList {
        pub fn new(capacity: i32) -> HistoryList {
            let lt: List = List::new();
            let cnt_mp: HashMap<i32, i32> = HashMap::new();
            let node_mp: HashMap<i32, Rc<RefCell<ListNode>>> = HashMap::new();
            return HistoryList {
                list: lt, 
                node_map: node_mp,
                cnt_map: cnt_mp,
                cap: capacity,
            }
        }
        
        // 使得hlist中key命中一次
        pub fn get(&mut self, key: i32) -> i32{
            if let Some(node) = self.node_map.get(&key) {
                let val = node.borrow().value;
                if let Some(count) = self.cnt_map.get_mut(&key) {
                    *count += 1;
                }
                return val
            }else {
                return -1;
            }
        }

        //往hlist塞数据
        pub fn put(&mut self, key: i32, value: i32) {
            if let Some(node) = self.node_map.get_mut(&key) {
                node.borrow_mut().value = value;
                if let Some(count) = self.cnt_map.get_mut(&key) {
                    *count += 1;
                }
            }else {
                if self.list.len == self.cap {
                    let k = self.list.remove_tail();
                    self.node_map.remove(&k);
                    self.cnt_map.remove(&k);
                }
                let new_node = ListNode::new(key, value);
                self.node_map.insert(key, new_node.clone());
                self.cnt_map.insert(key, 1);
                self.list.add_to_head(Some(new_node));
            }
        }

        //根据key值获取hlist中cnt_map中的cnt值
        pub fn get_cnt(&mut self, key: &i32) -> &i32{
            if let Some(cnt) = self.cnt_map.get(key) {
                return cnt;
            }
            return &-1;
        }

        pub fn show(&mut self) {
            self.list.show_all();
        }
    }

    impl LRUKCache {
        pub fn new(k: i32, hcapa: i32, lcapa: i32) -> LRUKCache {
            let lrul = LRUCache::new(lcapa);
            let hisl = HistoryList::new(hcapa);
            return LRUKCache { 
                k,
                hlist: hisl, 
                lrulist: lrul 
            }
        }

        //TODO
        pub fn get(&mut self, key: i32) -> i32 {
            let val = self.lrulist.get(key);
            if val != -1 {
                return val;
            } else {
                let val = self.hlist.get(key);
                if val == -1 {
                    return -1;
                } else {
                    let cnt = self.hlist.get_cnt(&key);
                    if *cnt == self.k {
                        if let Some(node) = self.hlist.node_map.get_mut(&key) {
                            let val = node.borrow().value;
                            self.hlist.list.remove_node(Some(node.clone()));
                            self.hlist.node_map.remove(&key);
                            self.hlist.cnt_map.remove(&key);
                            let new_node = ListNode::new(key, val);
                            self.lrulist.map.insert(key, new_node.clone());
                            if self.lrulist.list.len == self.lrulist.cap {
                                let k = self.lrulist.list.remove_tail();
                                self.lrulist.map.remove(&k);
                            }
                            self.lrulist.list.add_to_head(Some(new_node));
                        }
                    }
                    return val;
                }
            }
        }

        pub fn put(&mut self, key: i32, value: i32) {
            if let Some(node) = self.lrulist.map.get(&key) {
                self.lrulist.put(key, value);
            } else {
                self.hlist.put(key, value);
                let cnt = self.hlist.get_cnt(&key);
                if *cnt == self.k {
                    if let Some(node) = self.hlist.node_map.get(&key) {
                        let val = node.borrow().value;
                        self.hlist.list.remove_node(Some(node.clone()));
                        self.hlist.node_map.remove(&key);
                        self.hlist.cnt_map.remove(&key);
                        let new_node = ListNode::new(key, val);
                        self.lrulist.map.insert(key, new_node.clone());
                        if self.lrulist.list.len == self.lrulist.cap {
                            let k = self.lrulist.list.remove_tail();
                            self.lrulist.map.remove(&k);
                        }
                        self.lrulist.list.add_to_head(Some(new_node));
                    }
                }
            }
        }

        pub fn show(&mut self) {
            println!("LRUList:");
            self.lrulist.list.show_all();
            println!("HistoryList:");
            self.hlist.show();
        }

    }
}