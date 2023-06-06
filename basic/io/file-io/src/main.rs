use std::fs;
use std::fs::OpenOptions;
use std::io::prelude::*;

/* 一次性读入内存 */
fn read_once() {
    let text = fs::read_to_string("D:/1.txt").unwrap();
    println!("{}", text);
}

/* 读入缓冲区 */
fn read_buf() {
    let mut buffer = [0u8; 5];
    let mut file = fs::File::open("D:/1.txt").unwrap();

    file.read(&mut buffer).unwrap();
    println!("{:?}", buffer);

    file.read(&mut buffer).unwrap();
    println!("{:?}", buffer);
}

/* 一次性写入，文件存在则覆写（注意哦）；不存在则创建 */
fn write_once() {
    fs::write("D:/2.txt", "FROM RUST PROGRAM").unwrap();
}

/* File 类中不存在 append 静态方法，但是我们可以使用 OpenOptions 来实现用特定方法打开文件： */
fn write_append() -> std::io::Result<()> {
    let mut file = OpenOptions::new().append(true).open("D:/1.txt")?;
    file.write(b" APPEND WORD")?;
    Ok(())
}

/* 以读写权限打开文件，COVER 会追加到文件头部 */
fn write_cover() -> std::io::Result<()> {
    let mut file = OpenOptions::new().read(true).write(true).open("D:/1.txt")?;
    file.write(b"COVER")?;
    Ok(())
}

fn main() {
    // read_once()
    // read_buf()
    // write_once()
    // write_append();
    // write_cover();
}
