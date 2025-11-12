mod point_of_sale;

use std::{collections::HashMap, io};

use crate::point_of_sale::{Inventory, PointOfSale};

fn main() {
    let display = point_of_sale::Display {
        text: "".to_string(),
    };
    let inventory = Inventory::new(HashMap::from([("123456", 795), ("654321", 1000)]));
    let mut pos = PointOfSale::new(display, inventory, None);    

    loop {
        let mut barcode = String::new();
        println!("Enter barcode:");
        io::stdin()
            .read_line(&mut barcode)
            .expect("Failed to read line");

        barcode = barcode.trim().to_string();

        if barcode == "total" {
            break;
        }
        pos.on_barcode(&barcode);
    }
    pos.on_transaction_finished();
    }
