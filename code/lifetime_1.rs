fn main() {
    let z;   // lifetime 'a

    {
        let y = 1;  // lifetime 'b
        z = &y;
    }

    println!("{:?}", z);
}
