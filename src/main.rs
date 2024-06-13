use std::{io::{self, Read}, string};

fn main() {
   demo001()
}

fn demo001(){
    println!("Hello, world!");
    println!("Please enter a number: ");
    let mut input = String::new(); // 在这里我们创建了一个新的 String，用来接收下面的输入
    io::stdin()
        .read_line(&mut input) // 读取一行
        .expect("failed to read input"); // 比较粗暴的错误处理
    println!("Your input number is: {:?}.", input); // 打印输入的原始内容
    let number: i64 = input.trim().parse().expect("Input is not a number!"); // trim 把前后的空格、换行符这些空白字符都去掉，parse 将输入的字符串解析为 i64 类型，如果解析失败就报错
    println!("Your input is: {}.", number) // 打印 parse 之后的 i64 数字
}
