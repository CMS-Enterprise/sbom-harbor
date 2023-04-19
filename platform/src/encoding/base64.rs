pub use base64::*;

use base64::engine::general_purpose;

pub fn standard_encode(value: &str) -> String {
    general_purpose::STANDARD.encode(value)
}
