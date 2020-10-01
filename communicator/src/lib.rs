mod client; // 模块声明
mod network {
    fn connect() {}
    mod server {
        fn connect() {}
    }
}
// 记住使用  cargo build  而不是  cargo run  因为这是一个库 crate 而不是二进制 crate:
