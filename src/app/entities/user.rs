pub struct User {
    pub id: String,
    pub name: String,
    pub email: String,
    pub password: String
}

impl User {
    pub fn new(id: String, name: String, email: String, password: String) -> User {
        return User {
            id, name, email, password
        };
    }
}
