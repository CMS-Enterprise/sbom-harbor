use uuid::Uuid;

/// Generates a random alphanumeric string so we can use it
/// for temporary folders.
pub fn get_random_string() -> String {
    Uuid::new_v4().to_string().replace('-', "")
}
