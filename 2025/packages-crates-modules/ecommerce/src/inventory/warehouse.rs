pub struct Warehouse {
    name: String,
    location: String,
}

impl Warehouse {
    pub fn new(name: &str) -> Self {
        Warehouse {
            name: name.to_string(),
            location: "Unknown".to_string(),
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn set_location(&mut self, location: &str) {
        self.location = location.to_string();
    }
}
