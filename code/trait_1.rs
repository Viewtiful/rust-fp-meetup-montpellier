pub trait Printable {
    fn printable(&self) -> ();
}

impl Printable for i32 {
    fn printable(&self) -> () { println!("{:?}", self); }
}

fn main() {
    let a: i32 = 4;

    a.printable();
}