fn print_vec(vec: &mut Vec<i32>) -> () {
    println!("{:?}", vec);
    vec.push(3);
}

fn main() {
    let mut vec: Vec<i32> = Vec::new();
    vec.push(1);
    vec.push(2);

    print_vec(&mut vec);

    println!("{:?}", vec);    
}
