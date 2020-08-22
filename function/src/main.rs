fn main() {
    let c = "🍎123";
    let len = c.chars().count();
    let y = test();
    println!(" len: {}", y);
}

// 必须得指定返回类型
fn test() -> i32 {
    let x = 9;
    if x < 5 {
        x
    } else {
        0
    }
}
