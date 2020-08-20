fn main() {
    let c = "🍎123";
    let len = c.chars().count();
    println!("{}, len: {}", c, len);
    test();
}

// 必须得指定返回类型
fn test() -> i32 {
    5;
}