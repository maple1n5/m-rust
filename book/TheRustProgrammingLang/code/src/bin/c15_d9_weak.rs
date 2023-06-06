use std::cell::RefCell;
use std::rc::{Rc, Weak};

// 通过在 Node 定义中将子节点指向父节点的关系定义为一个 Weak<T> 引用
// 可以使父子节点在指向彼此的同时避免产生循环引用或内存泄漏
#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

// 有点混，可以理解为，我使用了谁的 rc，谁的 count + 1
// 这里的 RefCell 使用 Weak 引用：如果是 Rc 的话，父子节点就会循环引用
// Weak 节点即使 count > 0，当 StrongeCount = 0 时，依然销毁
// 当 main 结束，branch 离开作用域 rc = 0； 导致 node 被丢弃，即使 weakcount=0
fn main() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    // 1(自己);0
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );

    // None
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    // 1(自己);0
    println!(
        "branch strong = {}, weak = {}",
        Rc::strong_count(&branch),
        Rc::weak_count(&branch),
    );

    // 这里使 branch 的弱引用 + 1
    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    // 1(自己);1(leaf)
    println!(
        "branch strong = {}, weak = {}",
        Rc::strong_count(&branch),
        Rc::weak_count(&branch),
    );

    // 2(自己 + branch 创建时使用 leaf);0
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );

    // 获取父节点引用
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    // 2;0
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );
}
