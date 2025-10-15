// Tests we need:
// - product found -> output price   - Done
// - product not found   - Done
// - price is a price
// - invalid, empty string and null(?)  - Done
//
//
// Notes
// - We will use dollars

use std::collections::HashMap;

pub struct Display{
    pub text: String
}
impl Display{
    pub fn get_text(&self) -> &String{
        &self.text
    }
    fn set_text(&mut self, text: &str){
        self.text = text.to_string();
    }
    pub fn display_product_not_found(&mut self) {
        self.set_text("product not found");
    }
    pub fn display_no_barcode_read(&mut self) {
        self.set_text("error: no barcode read");
    }
    pub fn display_price(&mut self, price: &str) {
        self.set_text(price);
    }
}

pub struct PointOfSale{
    pub display: Display,
    pub inventory: Inventory,
}
impl PointOfSale {
    pub fn on_barcode(&mut self, barcode: &str){
        if barcode.is_empty() {
            self.display.display_no_barcode_read();
            return;
        }
        match self.inventory.get_price(barcode) {
            Some(price) => self.display.display_price(&price),
            None => self.display.display_product_not_found(),
        }
    }
}

pub struct Inventory{
    pub products: HashMap<&'static str, &'static str>,
}
impl Inventory{
    pub fn get_price(&self, barcode: &str) -> Option<String> {
        if self.product_found(barcode){
            Some(self.products[barcode].to_string())
        } else {
            None
        }
    }
    pub fn product_found(&self, barcode: &str) -> bool {
        self.products.contains_key(barcode)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn when_product_found_outputs_price() {
        let display = Display{text: "".to_string()};
        let inventory = HashMap::from([
            ("123456", "$7.95"),
            ("654321", "$6.50")
        ]);
        let mut pos = PointOfSale{display, inventory: Inventory{products: inventory}};
        pos.on_barcode("123456");
        assert_eq!(pos.display.get_text(), "$7.95");
    }

    #[test]
    fn when_other_product_found_outputs_different_price() {
        let display = Display{text: "".to_string()};
        let inventory = HashMap::from([
            ("123456", "$7.95"),
            ("654321", "$6.50")
        ]);
        let mut pos = PointOfSale{display, inventory: Inventory{products: inventory}};
        pos.on_barcode("654321");
        assert_eq!(pos.display.get_text(), "$6.50");
    }

    #[test]
    fn product_not_found() {
        let display = Display{text: "".to_string()};
        let inventory = HashMap::new();
        let mut pos = PointOfSale{display, inventory: Inventory{products: inventory}};
        pos.on_barcode("999999");
        assert_eq!(pos.display.get_text(), "product not found");
    }

    #[test]
    fn displays_error_on_empty_barcode() {
        let display = Display{text: "".to_string()};
        let inventory = HashMap::new();
        let mut pos = PointOfSale{display, inventory: Inventory{products: inventory}};
        pos.on_barcode("");
        assert_eq!(pos.display.get_text(), "error: no barcode read");
    }

}
