use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

// 多重所有权的可变数据
#[derive(Debug)]
enum List {
    // 这里的值用 Rc 可以被多次持有不可变引用，而 RefCell 又可以通过 borrow_mut() 来进行修改值
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

fn main() {
    let value = Rc::new(RefCell::new(5));

    // 持有一个 value 的引用
    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    // 持有 a 引用
    let b = Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));

    // 持有 a 引用
    let c = Cons(Rc::new(RefCell::new(10)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}
