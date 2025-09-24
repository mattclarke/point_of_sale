mod point_of_sale;

use std::io;

fn main() {
    
    let display = point_of_sale::Display{text: "".to_string()};
    let mut pos = point_of_sale::PointOfSale{display};
    let mut barcode = String::new();

    println!("Enter barcode:");
    io::stdin()
        .read_line(&mut barcode)
        .expect("Failed to read line");

    let barcode = barcode.trim();

    pos.on_barcode(&barcode);
    println!("{}", pos.display.get_text());

}
