#[derive(Debug)]
pub enum UserRole {
    Customer,
    Admin,
    Vendor,
}

pub struct User {
    email: String,
    name: String,
    role: UserRole,
    user_id: u32,
}

impl User {
    pub fn new(email: &str, name: &str, role: UserRole) -> Self {
        static mut NEXT_ID: u32 = 1;
        let id = unsafe {
            let id = NEXT_ID;
            NEXT_ID += 1;
            id
        };

        User {
            email: email.to_string(),
            name: name.to_string(),
            role,
            user_id: id,
        }
    }

    pub fn get_id(&self) -> u32 {
        self.user_id
    }

    pub fn get_email(&self) -> &str {
        &self.email
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_role(&self) -> &UserRole {
        &self.role
    }
}
