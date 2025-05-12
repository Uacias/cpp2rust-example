// tell Rust about the foreign C function:
unsafe extern "C" {
    unsafe fn add(a: i32, b: i32) -> i32;
}

fn main() {
    let a = 5;
    let b = 7;
    // unsafe because we're calling into C
    let sum = unsafe { add(a, b) };
    println!("{} + {} = {}", a, b, sum);
}
