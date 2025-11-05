use std::collections::HashMap;

pub struct Display {
    pub text: String,
}
impl Display {
    pub fn get_text(&self) -> &String {
        &self.text
    }
    fn set_text(&mut self, text: &str) {
        self.text = text.to_string();
    }
    pub fn display_product_not_found(&mut self) {
        self.set_text("product not found");
    }
    pub fn display_no_barcode_read(&mut self) {
        self.set_text("error: no barcode read");
    }
    pub fn display_price(&mut self, price: i32) {
        let price_as_string = format!("${}.{:0>2}", price / 100, price % 100);
        self.set_text(&price_as_string);
    }
}

pub struct PointOfSale {
    pub display: Display,
    pub inventory: Inventory,
    pub sales_tax: Option<f32>,
}
impl PointOfSale {
    pub fn on_barcode(&mut self, barcode: &str) {
        if barcode.is_empty() {
            self.display.display_no_barcode_read();
            return;
        }
        match self.inventory.get_price(barcode) {
            Some(price) => {
                let price = self.apply_tax(price);
                self.display.display_price(price)
            }
            None => self.display.display_product_not_found(),
        }
    }
    fn apply_tax(&mut self, price: i32) -> i32 {
        match self.sales_tax {
            Some(tax) => (price as f32 * (1.0 + tax)) as i32,
            None => price,
        }
    }
    fn on_transaction_finished(&mut self) {
        self.display.set_text("No sale in progress, please scan an item");
    }
}

pub struct Inventory {
    products: HashMap<&'static str, i32>,
}
impl Inventory {
    pub fn new(products: HashMap<&'static str, i32>) -> Inventory {
        Inventory { products: products }
    }
    pub fn get_price(&self, barcode: &str) -> Option<i32> {
        if self.product_found(barcode) {
            Some(self.products[barcode])
        } else {
            None
        }
    }
    fn product_found(&self, barcode: &str) -> bool {
        self.products.contains_key(barcode)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn when_product_found_outputs_price() {
        let display = Display {
            text: "".to_string(),
        };
        let inventory = HashMap::from([("123456", 795), ("654321", 650)]);
        let mut pos = PointOfSale {
            display,
            inventory: Inventory::new(inventory),
            sales_tax: None,
        };
        pos.on_barcode("123456");
        assert_eq!(pos.display.get_text(), "$7.95");
    }

    #[test]
    fn when_other_product_found_outputs_different_price() {
        let display = Display {
            text: "".to_string(),
        };
        let inventory = HashMap::from([("123456", 795), ("654321", 650)]);
        let mut pos = PointOfSale {
            display,
            inventory: Inventory::new(inventory),
            sales_tax: None,
        };
        pos.on_barcode("654321");
        assert_eq!(pos.display.get_text(), "$6.50");
    }

    #[test]
    fn product_not_found() {
        let display = Display {
            text: "".to_string(),
        };
        let mut pos = PointOfSale {
            display,
            inventory: Inventory::new(HashMap::new()),
            sales_tax: None,
        };
        pos.on_barcode("999999");
        assert_eq!(pos.display.get_text(), "product not found");
    }

    #[test]
    fn displays_error_on_empty_barcode() {
        let display = Display {
            text: "".to_string(),
        };
        let mut pos = PointOfSale {
            display,
            inventory: Inventory::new(HashMap::new()),
            sales_tax: None,
        };
        pos.on_barcode("");
        assert_eq!(pos.display.get_text(), "error: no barcode read");
    }

    #[test]
    fn displays_price_including_tax() {
        let tax = 0.2;
        let display = Display {
            text: "".to_string(),
        };
        let inventory = HashMap::from([("123456", 795), ("654321", 1000)]);
        let mut pos = PointOfSale {
            display,
            inventory: Inventory::new(inventory),
            sales_tax: Some(tax),
        };
        pos.on_barcode("654321");
        assert_eq!(pos.display.get_text(), "$12.00");
    }

    #[test]
    fn on_transaction_finished_with_zero_items() {
        let display = Display {
            text: "".to_string(),
        };
        let inventory = HashMap::new();
        let mut pos = PointOfSale {
            display,
            inventory: Inventory::new(inventory),
            sales_tax: None
        };
        pos.on_transaction_finished();
        assert_eq!(pos.display.get_text(), "No sale in progress, please scan an item");
    }
}
