pub struct User{
    pub username: String,
    first_name: String,
    last_name: String,
    // birthdate: time::Instant,
    age: u8,
}

impl User {
    pub fn new() -> User {
        User {
            username: "test".to_string(),
            first_name: "test".to_string(),
            last_name: "test".to_string(),
            age: 18,
        }
    }
}