fn main() {
    let a: *const u32 = &5;

    unsafe {
        println!("a is: {}", *a);
    }
}
