#![cfg_attr(feature = "axstd", no_std)]
#![cfg_attr(feature = "axstd", no_main)]

#[macro_use]
#[cfg(feature = "axstd")]
extern crate axstd as std;

use std::io::{self, Read};
use core::str;

#[cfg_attr(feature = "axstd", no_mangle)]
fn main() {
    println!("Please enter some text:");

    let mut buffer = [0; 1024]; // 创建一个缓冲区来存储读取的数据
    let stdin = io::stdin(); // 获取标准输入的句柄
    let mut handle = stdin.lock(); // 锁定stdin以读取数据

    // 读取数据到缓冲区，返回读取的字节数
    let bytes_read = handle.read(&mut buffer).unwrap();

    // 将读取的字节转换为字符串并打印
    let input_text = str::from_utf8(&buffer[..bytes_read])
        .expect("Failed to convert bytes to string");
    println!("You entered: {}", input_text);

}