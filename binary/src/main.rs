use wrapper;

fn main() {
    let num = 10;
    println!("Hello, world! {num} plus one is {}!", wrapper::add(num, 1));
}
