fn main() {
    let mut z;   // lifetime 'a


    let y = 1;  // lifetime 'b
    z = &y;


    println!("{:?}", z);
}
