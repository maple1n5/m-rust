use std::rc::Rc;
use crate::List::{Cons, Nil};

enum List {
    // Cons(i32, Box<List>),
    Cons(i32, Rc<List>),
    Nil,
}

fn main() {
    // 链表1： 2 -> 1 -> Nil
    // 链表2： 3 -> 链表 1
    // 链表3： 4 -> 链表 1

    //let list1 = Cons(2, Box::new(Cons(1, Box::new(Nil))));
    //let list2 = Cons(3, Box::new(list1));
    // use of moved value: `list1`
    // move occurs because `list1` has type `List`, which does not implement the `Copy` trait
    // 无法让两个列表同时持有另一列表的所有权
    //let list3 = Cons(4, Box::new(list1));

    // 使用 Reference count ，可以复制（不复制数据）一个不可变指针多次
    let list1 = Rc::new(Cons(2, Rc::new(Cons(1, Rc::new(Nil)))));
    let list2 = Cons(3, Rc::clone(&list1));
    let list3 = Cons(4, Rc::clone(&list1));

    // 这里使用的是 strong_count 强引用计数，和面还有个 weak_count（防止循环计数）
    println!("rc counting is :{}", Rc::strong_count(&list1))
}
