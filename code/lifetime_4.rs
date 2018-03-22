fn first_or_second(z: &i32, y: &i32) -> &i32 {
    if *z < 0 { y }
    else { z }
}


fn main() {
    let z = -1;   // lifetime 'a

    let y = 22;   // lifetime 'b

    println!("x: {:?}", first_or_second(&z, &y));
}
