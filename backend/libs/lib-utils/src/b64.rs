//! Base64 URL-safe encoding/decoding utilities.
//!
//! Uses URL-safe encoding without padding, suitable for tokens and URLs.

use crate::URL_SAFE_NO_PAD;
use base64::Engine;

/// Encode bytes to URL-safe base64 string (no padding).
///
/// # Example
/// ```
/// let encoded = b64u_encode(b"hello");
/// assert_eq!(encoded, "aGVsbG8");
/// ```
pub fn b64u_encode(content: impl AsRef<[u8]>) -> String {
    URL_SAFE_NO_PAD.encode(content)
}

/// Decode URL-safe base64 string to bytes.
///
/// # Errors
/// Returns error if input is not valid base64.
pub fn b64u_decode(b64u: &str) -> Result<Vec<u8>, base64::DecodeError> {
    URL_SAFE_NO_PAD.decode(b64u)
}

/// Decode URL-safe base64 string to UTF-8 string.
///
/// # Errors
/// Returns error if input is not valid base64 or not valid UTF-8.
pub fn b64u_decode_to_string(b64u: &str) -> Result<String, Error> {
    let bytes = b64u_decode(b64u)?;
    String::from_utf8(bytes).map_err(|_| Error::InvalidUtf8)
}

#[derive(Debug, thiserror::Error)]
/// Base64 decoding errors
pub enum Error {
    #[error("Base64 decode error: {0}")]
    Decode(#[from] base64::DecodeError),

    #[error("Invalid UTF-8 in base64 decoded data")]
    InvalidUtf8,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_b64u_encode_decode() {
        let original = b"Hello, World!";
        let encoded = b64u_encode(original);
        let decoded = b64u_decode(&encoded).unwrap();
        assert_eq!(original.as_slice(), decoded.as_slice());
    }

    #[test]
    fn test_b64u_decode_to_string() {
        let encoded = b64u_encode(b"test string");
        let decoded = b64u_decode_to_string(&encoded).unwrap();
        assert_eq!(decoded, "test string");
    }
}
