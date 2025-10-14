use super::products::Product;

#[derive(Debug)]
pub enum OrderStatus {
    Pending,
    Confirmed,
    Shipped,
    Delivered,
    Cancelled,
}

pub struct Order {
    order_id: u32,
    user_id: u32,
    items: Vec<(Product, u32)>, // (product, quantity)
    status: OrderStatus,
}

impl Order {
    pub fn new(user_id: u32) -> Self {
        static mut NEXT_ID: u32 = 1;
        let id = unsafe {
            let id = NEXT_ID;
            NEXT_ID += 1;
            id
        };

        Order {
            order_id: id,
            user_id,
            items: Vec::new(),
            status: OrderStatus::Pending,
        }
    }
    pub fn add_product(&mut self, product: Product, quantity: u32) {
        self.items.push((product, quantity));
    }

    pub fn calculate_total(&self) -> f64 {
        self.items
            .iter()
            .map(|(product, quantity)| product.get_price() * (*quantity as f64))
            .sum()
    }

    pub fn get_status(&self) -> &OrderStatus {
        &self.status
    }

    pub fn confirm(&mut self) {
        self.status = OrderStatus::Confirmed;
    }
}
