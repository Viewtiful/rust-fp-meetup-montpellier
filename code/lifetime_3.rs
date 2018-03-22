fn main() {
    let mut z = 1;   // lifetime 'a


    let y = z;  // lifetime 'b

    println!("y: {:?}", y);
    println!("z: {:?}", z);
}
