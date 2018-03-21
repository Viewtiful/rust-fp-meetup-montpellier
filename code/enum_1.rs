enum BarCode {
    Upc(i32, i32, i32, i32),
    QrCode(String),
}

fn main() {
    let bar_code = BarCode::QrCode(String::from("ABC"));
}
