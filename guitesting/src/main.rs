#[no_mangle]
pub extern "C" fn test(a:i32) -> i32 {
    return a*2;
}

fn main() {
    println!("Hello, world!");
    test(2);
    test2('t');
}

fn rusttest(a:char){
    println!("rust test ran: {a}");
}

#[no_mangle]
pub extern "C" fn test2(a:char){
    rusttest(a);
}
