fn main() {
    let vec: Vec<i32> = vec![1];
    let closure = move | x: i32 | {
        println!("x: {:?}, vec: {:?}", x, vec);
    };

    closure(1);

    println!("vec: {:?}", vec);
}
