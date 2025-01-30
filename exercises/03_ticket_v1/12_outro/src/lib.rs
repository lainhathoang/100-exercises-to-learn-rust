// TODO: Define a new `Order` type.
//   It should keep track of three pieces of information: `product_name`, `quantity`, and `unit_price`.
//   The product name can't be empty and it can't be longer than 300 bytes.
//   The quantity must be strictly greater than zero.
//   The unit price is in cents and must be strictly greater than zero.
//   Order must include a method named `total` that returns the total price of the order.
//   Order must provide setters and getters for each field.
//
// Tests are located in a different place this time—in the `tests` folder.
// The `tests` folder is a special location for `cargo`. It's where it looks for **integration tests**.
// Integration here has a very specific meaning: they test **the public API** of your project.
// You'll need to pay attention to the visibility of your types and methods; integration
// tests can't access private or `pub(crate)` items.
pub struct Order {
    product_name: String,
    quantity: usize,
    unit_price: isize,
}

impl Order {
    pub fn new(product_name: String, quantity: usize, unit_price: isize) -> Self {
        if product_name.len() > 300 || product_name.is_empty() {
            panic!("product_name can't be longer than 300 or empty");
        }

        if quantity <= 0 {
            panic!("quantity must be strictly greater than zero");
        }

        if unit_price <= 0 {
            panic!("unit_price must be strictly greater than zero")
        }

        Order {
            product_name,
            quantity,
            unit_price,
        }
    }

    pub fn total(&self) -> usize {
        self.quantity * self.unit_price as usize
    }

    pub fn product_name(&self) -> &str {
        self.product_name.as_str()
    }

    pub fn quantity(&self) -> &usize {
        &self.quantity
    }

    pub fn unit_price(&self) -> &isize {
        &self.unit_price
    }

    pub fn set_product_name(&mut self, new_product_name: String) {
        self.product_name = new_product_name;
    }

    pub fn set_quantity(&mut self, new_quantity: usize) {
        self.quantity = new_quantity;
    }

    pub fn set_unit_price(&mut self, new_unit_price: isize) {
        self.unit_price = new_unit_price;
    }
}
