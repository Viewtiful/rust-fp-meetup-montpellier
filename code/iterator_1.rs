fn main() {
    let vec = (1..)
        .filter(|x| x % 2 == 0)
        .take(3)
        .map(|x| x / 2)
        .collect::<Vec<i32>>();

    println!("{:?}", vec);
}
