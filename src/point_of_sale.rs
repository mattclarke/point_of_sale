use std::collections::HashMap;

pub struct Display {
    pub text: String,
}
impl Display {
    pub fn get_text(&self) -> &String {
        &self.text
    }
    fn display_text(&mut self, text: &str) {
        self.text = text.to_string();
        println!("{}", text);
    }
    pub fn display_product_not_found(&mut self) {
        self.display_text("product not found");
    }
    pub fn display_no_barcode_read(&mut self) {
        self.display_text("error: no barcode read");
    }
    pub fn display_price(&mut self, name: &str, price: i32) {
        let item_text = format!("{: <20}{: >10}", name, Self::format_price(price));
        self.display_text(&item_text);
    }
    pub fn display_no_sale(&mut self) {
        self.display_text("No sale in progress, please scan an item");
    }
    pub fn display_total(&mut self, total_amount: i32) {
        let total_amount_as_string = format!("{: <20}{: >10}", "Total:", Self::format_price(total_amount));
        self.display_text(&total_amount_as_string);
    }
    fn format_price(price: i32) -> String {
        return format!("${}.{:0>2}", price / 100, price % 100);
    }
}

pub struct PointOfSale {
    pub display: Display,
    pub inventory: Inventory,
    pub sales_tax: Option<f32>,
    pub total_amount: i32,
}
impl PointOfSale {
    pub fn new(display: Display, inventory: Inventory, sales_tax: Option<f32>) -> PointOfSale {
        PointOfSale {
            display,
            inventory,
            sales_tax,
            total_amount: 0,
        }
    }
    pub fn on_barcode(&mut self, barcode: &str) {
        if barcode.is_empty() {
            self.display.display_no_barcode_read();
            return;
        }
        match self.inventory.get_item(barcode) {
            Some(Product{price, name, ..}) => {
                let price = self.apply_tax(*price);
                self.total_amount += price;
                self.display.display_price(name, price);
            }
            None => self.display.display_product_not_found(),
        }
    }
    pub fn on_enter_manual_price(&mut self, price: &str) {
        let price = price.replace(".", "");
        let price = match price.parse::<i32>() {
            Ok(price) => price,
            Err(e) => {return;}
        };
        self.display.display_price("Manual entry", price);
    }
    fn apply_tax(&self, price: i32) -> i32 {
        match self.sales_tax {
            Some(tax) => (price as f32 * (1.0 + tax)) as i32,
            None => price,
        }
    }
    pub fn on_transaction_finished(&mut self) {
        if self.total_amount > 0 {
            self.display.display_total(self.total_amount);
        } else {
            self.display.display_no_sale();
        }
    }
    pub fn on_next_transaction(&mut self) {
        self.total_amount = 0;
    }
}

pub struct Inventory {
    products: HashMap<String, Product>,
}
impl Inventory {
    pub fn new(products: Vec<Product>) -> Inventory {
        let mut inventory = Inventory {
            products: HashMap::new(),
        };
        for product in products {
            inventory
                .products
                .insert(product.barcode.clone(), product.clone());
        }
        inventory
    }
    pub fn get_price(&self, barcode: &str) -> Option<i32> {
        if self.product_found(barcode) {
            Some(self.products[barcode].price)
        } else {
            None
        }
    }
    pub fn get_item(&self, barcode: &str) -> Option<&Product> {
        if self.product_found(barcode) {
            Some(&self.products[barcode])
        } else {
            None
        }
    }
    fn product_found(&self, barcode: &str) -> bool {
        self.products.contains_key(barcode)
    }
}

#[derive(Clone)]
pub struct Product {
    pub name: String,
    pub price: i32,
    pub barcode: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn standard() -> PointOfSale {
        let display = Display {
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
        PointOfSale::new(display, inventory, None)
    }

    fn assert_output(input: &str, expected: &str) {
        let words: Vec<&str> = input.split_whitespace().collect();
        assert_eq!(words.join(" "), expected);
    }

    #[test]
    fn when_product_found_outputs_item_name_and_price() {
        let mut pos = standard();
        pos.on_barcode("123456");
        assert_output(pos.display.get_text(), "Speedboat $7.95");
    }
    #[test]
    fn when_other_product_found_outputs_different_price() {
        let mut pos = standard();
        pos.on_barcode("654321");
        assert_output(pos.display.get_text(), "Rowboat $10.00");
    }

    #[test]
    fn product_not_found_shows_error_message() {
        let mut pos = standard();
        pos.on_barcode("999999");
        assert_eq!(pos.display.get_text(), "product not found");
    }

    #[test]
    fn displays_error_on_empty_barcode() {
        let mut pos = standard();
        pos.on_barcode("");
        assert_eq!(pos.display.get_text(), "error: no barcode read");
    }

    #[test]
    fn can_enter_price_manually_when_no_barcode() {
        let mut pos = standard();
        pos.on_enter_manual_price("12.34");
        assert_output(pos.display.get_text(), "Manual entry $12.34");
    }

    #[test]
    fn displays_price_including_tax() {
        let mut pos = standard();
        pos.sales_tax = Some(0.2);
        pos.on_barcode("654321");
        assert_output(pos.display.get_text(), "Rowboat $12.00");
    }

    #[test]
    fn on_transaction_finished_with_zero_items() {
        let mut pos = standard();
        pos.on_transaction_finished();
        assert_eq!(
            pos.display.get_text(),
            "No sale in progress, please scan an item"
        );
    }

    #[test]
    fn on_transaction_finished_with_one_item() {
        let mut pos = standard();
        pos.on_barcode("123456");
        pos.on_transaction_finished();
        assert_output(pos.display.get_text(), "Total: $7.95");
    }

    #[test]
    fn on_transaction_finished_with_3_items() {
        let mut pos = standard();
        pos.on_barcode("123456");
        pos.on_barcode("123456");
        pos.on_barcode("654321");
        pos.on_transaction_finished();
        assert_output(pos.display.get_text(), "Total: $25.90");
    }
    #[test]
    fn test_format_price() {
        assert_eq!(Display::format_price(750), "$7.50")
    }

    #[test]
    fn test_next_transaction_starts() {
        let mut pos = standard();
        pos.on_barcode("123456");
        pos.on_transaction_finished();
        pos.on_next_transaction();
        pos.on_barcode("123456");
        pos.on_transaction_finished();
        assert_output(pos.display.get_text(), "Total: $7.95");
    } 

}
