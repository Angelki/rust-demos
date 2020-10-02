mod gcd;

fn main() {
    // let i: &mut u64 = &mut 12;
    // let j: &mut u64 = &mut 16;
    let k: u64 = 12;
    let l: u64 = 16;
    // println!("{} {}", i, j);
    // gcd::swap(i, j);
    // println!("{} {}", i, j);
    println!("{}", gcd::gcd(k, l));
    println!("{}, {}", k, l);
}
