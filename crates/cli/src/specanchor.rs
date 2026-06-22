use std::{fs, path::Path};
use vaultcore_core::{
    decode_signed_specanchor, encode_signed_specanchor, sign_specanchor, SigningKeypair,
    SpecAnchor, VaultError, VerificationKey,
};

pub fn run(args: &[String]) -> Result<String, VaultError> {
    match args.first().map(String::as_str) {
        Some("generate") => generate(args),
        Some("verify") => verify(args),
        _ => Ok(usage()),
    }
}

fn generate(args: &[String]) -> Result<String, VaultError> {
    let out = value_after(args, "--out").unwrap_or("specanchor.signed");
    let signing_key = SigningKeypair::from_bytes([11; 32]);
    let builder_key = VerificationKey::from_bytes([22; 32])?;
    let payload = SpecAnchor::development_default(builder_key);
    let signed = sign_specanchor(payload, &signing_key)?;
    let encoded = encode_signed_specanchor(&signed)?;
    fs::write(out, encoded).map_err(io_error)?;
    Ok(format!("specanchor generated: {out}"))
}

fn verify(args: &[String]) -> Result<String, VaultError> {
    let input = value_after(args, "--in").unwrap_or("specanchor.signed");
    let bytes = fs::read(Path::new(input)).map_err(io_error)?;
    decode_signed_specanchor(&bytes)?;
    Ok(format!("specanchor verified: {input}"))
}

fn value_after<'a>(args: &'a [String], flag: &str) -> Option<&'a str> {
    args.windows(2)
        .find(|pair| pair[0] == flag)
        .map(|pair| pair[1].as_str())
}

fn usage() -> String {
    "usage: vaultcore-cli specanchor generate --out <path> | specanchor verify --in <path>"
        .to_owned()
}

fn io_error(error: std::io::Error) -> VaultError {
    let message = error.to_string();
    drop(error);
    VaultError::new(
        vaultcore_core::VaultErrorCode::SpecAnchorFailure,
        None,
        format!("SpecAnchor file operation failed: {message}"),
    )
}
