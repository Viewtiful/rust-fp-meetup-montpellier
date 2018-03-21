#[derive(Debug)]
struct Book {
    name: String,
    page_number: u32
}

fn main() {
    let mut library: Vec<Book> = Vec::with_capacity(3);

    library.push(Book { name: "To Kill a Mockingbird".to_string(), page_number: 324 });
    library.push(Book { name: "The Little Prince".to_string(), page_number: 93 });
    library.push(Book { name: "1984".to_string(), page_number: 328 });

    println!("{:?}", library);
}