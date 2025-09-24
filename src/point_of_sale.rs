// Tests we need:
// - product found -> output price
// - product not found   - Done
// - price is a price
// - invalid, empty string and null(?)  !! Next time
// -
//
//
// Notes
// - We will use dollars

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
}

pub struct PointOfSale{
    pub display: Display,
}
impl PointOfSale {
    pub fn on_barcode(&mut self, barcode: &str){
        if barcode.is_empty() {
            self.display.set_text("error: no barcode read");
            return;
        }
        if barcode == "123456"{
            self.display.set_text("$7.95");
        } else if barcode == "654321" {
            self.display.set_text("$6.50");
        } else {
            self.display.set_text("product not found");
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn when_product_found_outputs_price() {
        let display = Display{text: "".to_string()};
        let mut pos = PointOfSale{display};
        pos.on_barcode("123456");
        assert_eq!(pos.display.get_text(), "$7.95");
    }

    #[test]
    fn when_other_product_found_outputs_different_price() {
        let display = Display{text: "".to_string()};
        let mut pos = PointOfSale{display};
        pos.on_barcode("654321");
        assert_eq!(pos.display.get_text(), "$6.50");
    }

    #[test]
    fn product_not_found() {
        let display = Display{text: "".to_string()};
        let mut pos = PointOfSale{display};
        pos.on_barcode("999999");
        assert_eq!(pos.display.get_text(), "product not found");
    }

    #[test]
    fn displays_error_on_empty_barcode() {
        let display = Display{text: "".to_string()};
        let mut pos = PointOfSale{display};
        pos.on_barcode("");
        assert_eq!(pos.display.get_text(), "error: no barcode read");
    }

}
