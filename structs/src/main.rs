// struct User {
//     username: String,
//     email: String,
//     sign_in_count: u64,
//     active: bool,
// }

// fn main() {
//     let mut user1 = User {
//         email: String::from("xx@gmail.com"),
//         username: String::from("someusername123"),
//         active: true,
//         sign_in_count: 1,
//     };
//     // println!("Hello, world!");
//     user1.email = String::from("anotheremail@example.com");
// }

struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn distance(&self, other: &Point) -> f64 {
        let x_squared = f64::powi(other.x - self.x, 2);
        let y_squared = f64::powi(other.y - self.y, 2);
        f64::sqrt(x_squared + y_squared)
    }
}

fn main() {
    let p1 = Point { x: 0.0, y: 0.0 };
    let p2 = Point { x: 5.0, y: 6.5 };
    p1.distance(&p2);
    (&p1).distance(&p2);
}
