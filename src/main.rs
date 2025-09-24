mod point_of_sale;

use std::{collections::HashMap, io};

use crate::point_of_sale::PointOfSale;

fn main() {
    
    let display = point_of_sale::Display{text: "".to_string()};
    let inventory = HashMap::from([
        ("123456", "$7.95"),
        ("654321", "$6.50")
    ]);
    let mut pos = PointOfSale{display, inventory};
    let mut barcode = String::new();

    println!("Enter barcode:");
    io::stdin()
        .read_line(&mut barcode)
        .expect("Failed to read line");

    let barcode = barcode.trim();

    pos.on_barcode(&barcode);
    println!("{}", pos.display.get_text());

}
