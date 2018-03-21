fn main() {
    let n = Some(3);

    match n {
        Some(3) => println!("Got a 3!"),
        Some(_) => println!("Got something else!"),
        None => println!("Got nothing :("),
    }
}