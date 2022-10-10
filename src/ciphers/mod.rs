mod aes;
mod another_rot13;
mod base64;
mod caesar;
mod chacha;
mod hashing_traits;
mod morse_code;
mod polybius;
mod rot13;
mod salsa;
mod sha256;
mod tea;
mod theoretical_rot13;
mod transposition;
mod vigenere;
mod xor;

pub use self::aes::{aes_decrypt, aes_encrypt, AesKey};
pub use self::another_rot13::another_rot13;
pub use self::base64::{base64_decode, base64_encode};
pub use self::caesar::caesar;
pub use self::chacha::chacha20;
pub use self::hashing_traits::Hasher;
pub use self::hashing_traits::HMAC;
pub use self::morse_code::{decode, encode};
pub use self::polybius::{decode_ascii, encode_ascii};
pub use self::rot13::rot13;
pub use self::salsa::salsa20;
pub use self::sha256::SHA256;
pub use self::tea::{tea_decrypt, tea_encrypt};
pub use self::theoretical_rot13::theoretical_rot13;
pub use self::transposition::transposition;
pub use self::vigenere::vigenere;
pub use self::xor::xor;
