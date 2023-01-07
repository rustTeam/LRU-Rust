# LRU-Rust
本作业实现了两个算法，分别是LRU算法与LRU-K算法

## LRU算法简介

### 原理

​	LRU(Least recently used，最近最少使用)算法根据数据的历史访问记录来进行淘汰数据。其核心思想是"如果数据最近被访问过，那么将来被访问的概率也会更高"。

### 实现

​	最常见的实现使用一个链表保存缓存数据，详细算法实现如下：

* 新数据插入链表头部
* 每当缓存命中时（即缓存数据被访问），则将数据移到链表头部
* 当链表满时，将链表尾部的数据丢弃

## LRU-K算法简介

### 原理

​	LRU-K中的K代表最近使用的次数，因此LRU可以认为是LRU-1。LRU-K的主要目的是为了解决LRU算法"缓存污染"的问题。其核心思想是将"最近使用过1次"的判断标准扩展为"最近使用K次"。

### 实现

​	相比LRU，LRU-K需要多维护一个队列，用于记录所有缓存数据被访问的历史。只有当数据的访问次数达到K次的时候，才将数据放入缓存。当需要淘汰数据时，LRU-K会淘汰第K次访问时间距当前时间最大的数据。详细实现如下：

* 数据第一次被访问，加入到访问历史列表
* 如果数据在访问历史列表里后没有达到K次访问，则按照FIFO淘汰
* 当访问历史队列中的数据访问次数达到K次后，将数据索引从历史队列删除，将数据移到缓存队列中，并缓存此数据，缓存队列按LRU的方式淘汰数据
* 缓存数据队列中被再次访问后，按LRU的方式淘汰数据
* 需要淘汰数据时，淘汰缓存队列中排在末尾的数据

## 数据结构简介

#### 双向链表节点

```rust
pub struct ListNode {
    key: i32,  //链表节点唯一对应的key值
    value: i32, //链表节点的value
    next: Option<Rc<RefCell<ListNode>>>,  //指向下一个链表节点的指针
    prev: Option<Rc<RefCell<ListNode>>>   //指向前一个链表节点的指针
}
```

#### 链表数据结构

```rust
pub struct List {
    len: i32,  //链表长度
    pub head: Option<Rc<RefCell<ListNode>>>,  //链表头
    pub tail: Option<Rc<RefCell<ListNode>>>   //链表尾
}
```

#### LRU缓存列表数据结构

```rust
pub struct LRUCache {
    map: HashMap<i32, Rc<RefCell<ListNode>>>,  //用于快速访问链表节点的哈希表
    pub list: List,  //用于存放缓存数据的双向链表
    cap: i32,  //缓存列表的上限值
}
```

#### 历史列表

```rust
// 基于FIFO的历史列表
pub struct HistoryList {
    list: List,  //用于存放历史访问数据的历史列表
    node_map: HashMap<i32,Rc<RefCell<ListNode>>>,  //用于快速访问链表节点的哈希表
    cnt_map: HashMap<i32, i32>,  //用于快速访问对应链表节点访问次数的哈希表
    cap: i32,  //历史列表的上限值
}
```

#### LRU-K缓存列表

```rust
pub struct LRUKCache {
    k: i32,   //LRU-K中对应的k值
    hlist: HistoryList,  //历史列表
    lrulist: LRUCache,  //LRU缓存列表
}
```

