use std::io;

fn read_and_print() {
    // 读取输入
    let mut input = String::new();
    println!("Please input something:");

    io::stdin().read_line(&mut input).expect("cant't read");

    // 输出输入
    println!("Your input is: {}", input);
}

fn main() {
    read_and_print();
}