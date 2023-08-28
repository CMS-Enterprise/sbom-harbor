

pub struct Token {
    pub key: String,
    pub value: String,
}

impl Token {
    pub fn new(value: String) -> Self {
        Self {
            key: String::from("Authorization"),
            value
        }
    }

    pub fn new_with_header_name(key: String, value: String) -> Self {
        Self {
            key,
            value,
        }
    }
}