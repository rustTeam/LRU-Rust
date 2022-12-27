mod lib;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use lib::double_list::HistoryList;
use lib::double_list::LRUCache;
use lib::double_list::ListNode;
use lib::double_list::List;
use lib::double_list::LRUKCache;

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
    println!("{}", list.head.is_none());
    list.add_to_head(node1);
    println!("{}", list.head.is_none());
    list.add_to_head(node2);
    list.add_to_head(node3);
    list.show_all();
    let x = 1;
    if let Some(t) = mp.get(&x) {
        list.move_to_head(t.clone());
    }
    list.show_all();
}
fn test2(){
    let mut lru_cache = LRUCache::new(5);
    let ans = lru_cache.get(0);
    lru_cache.put(5,1);
    lru_cache.put(6, 2);
    lru_cache.get(5);
    lru_cache.put(7, 3);
    lru_cache.put(8, 4);
    lru_cache.put(7, 8);
    lru_cache.get(8);
    lru_cache.put(1, 8);
    lru_cache.put(2, 4);
    lru_cache.list.show_all();
}

fn test_history_list() {
    let mut hlist = HistoryList::new(4);
    hlist.put(1, 1);
    hlist.put(2, 2);
    hlist.put(3, 3);
    hlist.put(4, 4);
    hlist.put(5, 5);
    hlist.put(2, 5);
    hlist.put(2, 4);
    let key = 2;
    let cnt = hlist.get_cnt(&key);
    println!("cnt: {}", cnt);
    let key = 1;
    let cnt = hlist.get_cnt(&key);
    println!("cnt: {}", cnt);
    hlist.show();
}

fn test3() {
    let mut lrukCache = LRUKCache::new(2,5,6);
    lrukCache.put(1,1);
    lrukCache.put(2,2);
    lrukCache.put(3,3);
    lrukCache.put(4,4);
    lrukCache.put(5,5);
    lrukCache.get(4);
    lrukCache.get(3);
    lrukCache.get(2);
    lrukCache.get(4);
    lrukCache.put(6,6);
    lrukCache.put(7,7);
    lrukCache.put(8,8);
    lrukCache.put(9,5);
    lrukCache.put(7,3);
    lrukCache.put(6,1);
    lrukCache.put(2,1);
    lrukCache.put(10,12);
    lrukCache.put(13,9);
    lrukCache.put(1,1);
    lrukCache.show();

}
fn main() {
    //test2();
    //test_history_list();
    test3();
    //println!("Hello, world!");
}
