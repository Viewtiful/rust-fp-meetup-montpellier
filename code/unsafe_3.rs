use std::ptr;

fn main() {
    let mut a = &mut 5 as *mut i32;

    unsafe {
        a = ptr::null_mut();
        println!("a is {}", *a);
    }
}
