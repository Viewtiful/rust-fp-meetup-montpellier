fn print_vec(vec: Vec<i32>) -> Vec<i32> {
    println!("{:?}", vec);

    vec
}

fn main() {
    let mut vec: Vec<i32> = Vec::new();
    vec.push(1);
    vec.push(2);

    let vec: Vec<i32> = print_vec(vec);

    println!("{:?}", vec);    
}
