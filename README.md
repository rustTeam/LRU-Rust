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

## 具体函数介绍
### 双向链表节点ListNode
####  new( )函数
```rust
//ListNode构造函数，初始化双向链表节点，包括键、值、前后指针
pub fn new(k: i32, v: i32) -> Rc<RefCell<ListNode>> {
    ......
}
```

####  display( )函数
```rust
//输出节点的键、值
pub fn display(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result  {
    ......
}
```

### 链表List
####  new( )函数
```rust
//List构造函数，初始化链表，包括初始长度0、以双向链表节点ListNode为基础的head头结点指针、tail尾结点指针
pub fn new() -> List {
    ......
}
```

####  add_to_head( )函数
```rust
//将输入的结点加入到链表队首
pub fn add_to_head(&mut self, node: Option<Rc<RefCell<ListNode>>>) {
    ......
}
```

####  remove_node( )函数
```rust
//将输入的结点从链表移出
pub fn remove_node(&mut self, node: Option<Rc<RefCell<ListNode>>>) {
    ......
}
```

####  move_to_head( )函数
```rust
//将输入的结点从链表原先的位置移至链表队首位置，由remove_node()函数和add_to_head()函数组合实现
pub fn move_to_head(&mut self, node: Option<Rc<RefCell<ListNode>>>) {
    ......
}
```

####  remove_tail( )函数
```rust
//使用remove_node()函数去除队尾节点，返回被去除的队尾节点的键
pub fn remove_tail(&mut self) -> i32 {
    ......
}
```

####  show_all( )函数
```rust
//让目前链表里除了head、tail节点外的所有节点按序展示节点信息（键、值）
pub fn show_all(&mut self) {
    ......
}
```

### LRU缓存列表LRUCache
####  new( )函数
```rust
//LRUCache构造函数，初始化LRU缓存列表，包括链表list、缓存列表容量capacity、储存键与节点映射关系的哈希表map
pub fn new(capacity: i32)  -> LRUCache {
    ......
}
```

####  get( )函数
```rust
//get()函数，通过键值来获取缓存列表中对应数据
pub fn get(&mut self, key: i32) -> i32 {
    ......
}
```

####  put( )函数
```rust
//put()函数，通过键值来更新缓存列表中对应数据
pub fn put(&mut self, key: i32, val: i32) {
    ......
}
```

### 历史列表HistoryList
####  new( )函数
```rust
//HistoryList构造函数，初始化历史列表，包括链表list、缓存列表容量cap、储存键与节点映射关系的哈希表node_map、储存键与访问次数关系的哈希表cnt_map
pub fn new(capacity: i32) -> HistoryList {
    ......
}
```

####  get( )函数
```rust
//get()函数，通过键值来访问HistoryList中的对应数据
pub fn get(&mut self, key: i32) -> i32 {
    ......
}
```

####  put( )函数
```rust
//put()函数，通过键值来更新HistoryList中的对应数据
pub fn put(&mut self, key: i32, value: i32) {
    ......
}
```

####  get_cnt( )函数
```rust
//根据key值获取hlist中cnt_map中的cnt值
pub fn get_cnt(&mut self, key: &i32) -> &i32 {
    ......
}
```

####  show( )函数
```rust
//展示HistoryList里所有节点的信息
pub fn show(&mut self) {
    ......
}
```

### LRU-K缓存列表LRUKCache
####  new( )函数
```rust
//LRUKCache构造函数，初始化LRU-K缓存列表，包括访问次数阈值k、历史队列hlist、缓存队列lrulist
pub fn new(k: i32, hcapa: i32, lcapa: i32) -> LRUKCache {
    ......
}
```

####  get( )函数
```rust
//get()函数，通过键值访问对应数据，先去lrulist里寻找，若找不到再去hlist
pub fn get(&mut self, key: i32) -> i32 {
    ......
}
```

####  put( )函数
```rust
//put()函数，通过键值访问对应数据，先去lrulist里寻找，若找不到再去hlist
pub fn put(&mut self, key: i32, value: i32) {
    ......
}
```

####  show( )函数
```rust
//依序展示lrulist和hlist里的节点信息
pub fn show(&mut self) {
    ......
}
```