#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlatformCapability {
    Available,
    Unsupported,
}

#[must_use]
pub const fn passkey_capability() -> PlatformCapability {
    PlatformCapability::Available
}

#[must_use]
pub const fn biometric_capability() -> PlatformCapability {
    PlatformCapability::Available
}
