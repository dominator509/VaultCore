use zeroize::{Zeroize, ZeroizeOnDrop};

#[derive(Clone, PartialEq, Eq, Zeroize, ZeroizeOnDrop)]
pub struct SealedBytes(Vec<u8>);

impl SealedBytes {
    #[must_use]
    pub fn new(bytes: Vec<u8>) -> Self {
        Self(bytes)
    }

    #[must_use]
    pub fn expose_for_builder_only(&self) -> &[u8] {
        &self.0
    }

    #[must_use]
    pub fn into_inner_for_builder_only(mut self) -> Vec<u8> {
        let mut out = Vec::new();
        std::mem::swap(&mut self.0, &mut out);
        out
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl From<Vec<u8>> for SealedBytes {
    fn from(value: Vec<u8>) -> Self {
        Self::new(value)
    }
}

impl std::fmt::Debug for SealedBytes {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("SealedBytes([redacted])")
    }
}

#[cfg(test)]
mod tests {
    use super::SealedBytes;

    #[test]
    fn sealed_bytes_do_not_expose_in_debug() {
        let sealed = SealedBytes::new(b"plaintext".to_vec());
        let rendered = format!("{sealed:?}");
        assert!(!rendered.contains("plaintext"));
    }
}
