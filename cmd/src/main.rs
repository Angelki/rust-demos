use std::io::Write;
use std::str::FromStr;

pub fn swap<T: Clone>(a: &mut T, b: &mut T) {
    let t = a.clone();
    *a = b.clone();
    *b = t;
}

pub fn gcd(mut m: u64, mut n: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if n > m {
            swap::<u64>(&mut m, &mut n);
        }
        m = m % n;
    }
    n
}

fn main() {
    let mut numbers = Vec::new();
    // 读取命令行参数参数 跳过程序的名称 剩下的塞进numbers
    let args = std::env::args();
    for arg in args.skip(1) {
        numbers.push(u64::from_str(&arg).expect("参数类型错误"));
    }
    if numbers.len() == 0 {
        writeln!(std::io::stderr(), "请输入两个数字作为参数").unwrap();
        std::process::exit(1);
    }
    let mut d = numbers[0];
    for m in &numbers[1..] {
        d = gcd(d, *m);
    }
    println!("{:?}的最大公约数是{}", numbers, d);
}
