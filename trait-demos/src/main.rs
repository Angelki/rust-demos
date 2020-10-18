fn main() {
    let tweet = Tweet {
        username: String::from("Rust从入门到放弃"),
        content: String::from("什么鬼"),
        reply: false,
        retweet: false,
    };
    println!("内容: {}", tweet.summary());
}
// 定义一个trait 类似于interface 公有的
pub trait Summarizable {
    // 申明
    fn author_summary(&self) -> String;
    // summary已经实现了
    fn summary(&self) -> String {
        format!("(Read more from {}...)", self.author_summary())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

// impl Summarizable for NewsArticle {
//     fn summary(&self) -> String {
//         format!("{}, by {} ({})", self.headline, self.author, self.location)
//     }
// }

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summarizable for Tweet {
    fn author_summary(&self) -> String {
        format!("@{}", self.username)
    }
}
