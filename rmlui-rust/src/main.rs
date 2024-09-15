#[no_mangle]
pub extern "C" fn test(a:i32) -> i32 {
    a*2
}

fn main() {
    println!("Hello, world!");
    println!("{}", test(2));
}
