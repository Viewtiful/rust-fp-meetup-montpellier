fn print_vec(vec: Vec<i32>) {
    println!("{:?}", vec);
}

fn main() {
    let mut vec: Vec<i32> = Vec::new();
    vec.push(1);
    vec.push(2);

    print_vec(vec);

    println!("{:?}", vec);    
}
