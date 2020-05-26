use nth_prime::nth;

pub fn main() {
    let n: u32 = 3;
    let p: u32 = nth(n);

    println!("{}", p);
}