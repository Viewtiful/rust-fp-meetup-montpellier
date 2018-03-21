fn main() {
    let mut a = &mut 5 as *mut i32;

    unsafe {
        println!("a is: {}", *a);
    }
}
