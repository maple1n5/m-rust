use std::ops::Deref;

// 模拟 Box<T>
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(t: T) -> MyBox<T> {
        MyBox(t)
    }
}

impl<T> Deref for MyBox<T> {
    // Deref trait 的关联类型
    type Target = T;

    fn deref(&self) -> &Self::Target {
        // 返回一个指向值的引用
        &self.0
    }
}

fn hello(name: &str) {
    println!("Hello, {}", name)
}

fn main() {
    let i = 5;
    let j = &i;
    let k = Box::new(i);

    assert_eq!(5, i);

    // no implementation for `{integer} == &{integer}`
    // assert_eq!(5, j);

    assert_eq!(5, *j);
    // *(xx.deref())，
    assert_eq!(5, *(j.deref()));

    // 对 box 进行解引用
    assert_eq!(5, *k);

    // 使用模拟的 MyBox<T>
    let a = MyBox::new(5);

    // 报错：type `MyBox<{integer}>` cannot be dereferenced
    // assert_eq!(5, *a);
    
    // 实现了 Deref trait 后
    assert_eq!(5, *a);

    // ------------------------
    hello("Allen");

    let name = Box::new(String::from("Bob"));
    // 函数、方法的自动解引用
    hello(&name);

    // 如果没有自动解引用
    hello(&(*name)[..])

}
