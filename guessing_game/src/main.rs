extern crate rand;

use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("我们来玩猜数字的游戏吧！");
    let secret_number = rand::thread_rng().gen_range(1, 101);
    // println!("The secret number is: {}", secret_number);
    loop {
        println!("请输入你的猜测：");
        let mut guess = String::new();
        //  如果不添加expect warn：this `Result` may be an `Err` variant, which should be handled
        io::stdin().read_line(&mut guess).expect("读取输入失败！");
        // 这里创建了一个叫做  guess  的变量。不过等等,不是已经有了一个叫做  guess 的变量了吗?
        // 确实如此,不过 Rust 允许隐藏(shadow),用一个新值来隐藏  guess  之前的值。
        // let guess: u32 = guess.trim().parse().expect("请输入一个数字！！");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("你猜测的值为： {}", guess);
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("猜小了"),
            Ordering::Greater => println!("猜大了"),
            Ordering::Equal => {
                println!("恭喜你，你赢了");
                break;
            }
        }
    }
}
