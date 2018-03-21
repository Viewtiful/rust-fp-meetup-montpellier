pub trait Printable {
    fn printable(&self) -> ();
}

impl Printable for i32 {
    fn printable(&self) -> () { println!("{:?}", self); }
}

fn need_printable<T: Printable>(value: T) -> () {
    value.printable();
}

fn main() {
    let a: i32 = 4;

    need_printable(a);
}
