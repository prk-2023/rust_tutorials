#[derive(Debug, Clone)]

pub struct Product {
    name: String,
    price: f64,
    stock: u32,
}

impl Product {
    pub fn new(name: &str, price: f64, stock: u32) -> Self {
        Product {
            name: name.to_string(),
            price,
            stock,
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_price(&self) -> f64 {
        self.price
    }

    pub fn get_stock(&self) -> u32 {
        self.stock
    }

    pub fn reduce_stock(&mut self, quantity: u32) -> Result<(), String> {
        if quantity > self.stock {
            Err("Not enough stock".to_string())
        } else {
            self.stock -= quantity;
            Ok(())
        }
    }
}
