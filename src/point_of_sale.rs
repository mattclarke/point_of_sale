// Tests we need:
// - product found -> output price
// - product not found
// - price is a price
// - invalid, empty string and null(?)
// -
//
//
// Notes
// - We will use dollars

struct Display{}
impl Display{
    fn get_text(&self) -> String{
        "$7.95".to_string()
    }
}

struct PointOfSale{}
impl PointOfSale {
    fn on_barcode(&self, barcode: &str){

    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn when_product_found_outputs_price() {
        let display = Display{};
        let pos = PointOfSale{};
        pos.on_barcode("123456");
        assert_eq!(display.get_text(), "$7.95");
    }
}
