pub use base64::*;

use base64::engine::general_purpose;

/// Encode arbitrary octets as base64 using the general purpose standard engine.
pub fn standard_encode(value: &str) -> String {
    general_purpose::STANDARD.encode(value)
}
