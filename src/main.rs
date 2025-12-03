mod point_of_sale;

use std::{collections::HashMap, io};

use crate::point_of_sale::{Inventory, PointOfSale, Product};

fn main() {
    let display = point_of_sale::Display {
        text: "".to_string(),
    };
    let inventory = Inventory::new(vec![
        Product {
            name: "Speedboat".to_string(),
            price: 795,
            barcode: "123456".to_string(),
        },
        Product {
            name: "Rowboat".to_string(),
            price: 1000,
            barcode: "654321".to_string(),
        },
    ]);
    let mut pos = PointOfSale::new(display, inventory, None);

    'outer: loop {
        pos.on_next_transaction();
        loop {
            let mut barcode = String::new();
            println!("Enter barcode:");
            io::stdin()
                .read_line(&mut barcode)
                .expect("Failed to read line");

            barcode = barcode.trim().to_string();

            if barcode == "finish" {
                pos.on_transaction_finished();
                break;
            }
            if barcode == "exit" {
                pos.on_transaction_finished();
                break 'outer;
            }
            pos.on_barcode(&barcode);
        }
        println!("=================");
    }
}
