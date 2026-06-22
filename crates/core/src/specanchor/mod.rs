pub mod schema;
pub mod verify;

pub use schema::{SignedSpecAnchor, SpecAnchor, SpecAnchorCryptoSuite};
pub use verify::{
    decode_signed_specanchor, encode_signed_specanchor, sign_specanchor, verify_signed_specanchor,
};
