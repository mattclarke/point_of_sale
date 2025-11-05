mod point_of_sale;

use std::{collections::HashMap, io};

use crate::point_of_sale::{Inventory, PointOfSale};

fn main() {
    let display = point_of_sale::Display {
        text: "".to_string(),
    };
    let inventory = HashMap::from([("123456", 795), ("654321", 650)]);
    let mut pos = PointOfSale {
        display,
        inventory: Inventory::new(inventory),
        sales_tax: None,
        total_amount: 0,
    };
    let mut barcode = String::new();

    println!("Enter barcode:");
    io::stdin()
        .read_line(&mut barcode)
        .expect("Failed to read line");

    let barcode = barcode.trim();

    pos.on_barcode(&barcode);
    println!("{}", pos.display.get_text());
}
