use test_rust::add;
use test_rust::div;
use test_rust::mul;
use test_rust::sub;

fn main() {
    println!("Hello, worl!");
    println!("add(1, 2) = {} ", add(1, 2));
    println!("sub(1, 2) = {} ", sub(1, 2));
    println!("mul(1, 2) = {} ", mul(1, 2));
    println!("div(1, 2) = {} ", div(1, 2));
}
