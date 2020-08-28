enum Coin {
    Penny(i32),
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny(count) => {
            println!("Lucy Penny: {}", count);
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn main() {
    let value = Coin::Penny(3);
    value_in_cents(value);
}
