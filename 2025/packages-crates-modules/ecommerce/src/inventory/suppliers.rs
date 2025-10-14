pub struct Supplier {
    name: String,
    contact_email: String,
}

impl Supplier {
    pub fn new(name: &str, email: &str) -> Self {
        Supplier {
            name: name.to_string(),
            contact_email: email.to_string(),
        }
    }
}
