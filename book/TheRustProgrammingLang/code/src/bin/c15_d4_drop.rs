use std::mem::drop;

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    // 注意：先创建的，后丢弃
    fn drop(&mut self) {
        println!("CustomSmartPointer drop with data: {}", self.data);
    }
}

fn main() {
    let p1 = CustomSmartPointer {
        data: String::from("first create"),
    };

    let p2 = CustomSmartPointer {
        data: String::from("second create"),
    };

    // 如果需要提前 drop，而不是等到代码出作用域，此时需要使用 std::mem::drop
    drop(p1);

    println!("p1 & p2 created!");
}
