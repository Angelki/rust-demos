fn main() {
    let mut s = String::from("Hello, world!");
    let value = first_word(&s);
    println!("value: {}", value);
    println!("value: {}", s);
    let hello = &s[0..value];
    println!("value: {}", hello);
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            println!("bytes: {:?}", bytes);
            return i;
        }
    }
    s.len()
}
