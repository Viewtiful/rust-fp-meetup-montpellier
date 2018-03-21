fn main() {
    let closure = | x | println!("{:?}", x);

    closure(3);
    closure("Hey!");
}
