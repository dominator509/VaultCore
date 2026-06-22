use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SecretType {
    #[serde(rename = "API_KEY")]
    ApiKey,
    #[serde(rename = "LOGIN")]
    Login,
    #[serde(rename = "OAUTH_APP")]
    OAuthApp,
    #[serde(rename = "SSH_KEY")]
    SshKey,
    #[serde(rename = "WALLET_KEY")]
    WalletKey,
    #[serde(rename = "CERT")]
    Cert,
    #[serde(rename = "NOTE")]
    Note,
    #[serde(rename = "BLOB")]
    Blob,
}

impl SecretType {
    pub const ALL: [Self; 8] = [
        Self::ApiKey,
        Self::Login,
        Self::OAuthApp,
        Self::SshKey,
        Self::WalletKey,
        Self::Cert,
        Self::Note,
        Self::Blob,
    ];

    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ApiKey => "API_KEY",
            Self::Login => "LOGIN",
            Self::OAuthApp => "OAUTH_APP",
            Self::SshKey => "SSH_KEY",
            Self::WalletKey => "WALLET_KEY",
            Self::Cert => "CERT",
            Self::Note => "NOTE",
            Self::Blob => "BLOB",
        }
    }
}

impl fmt::Display for SecretType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::SecretType;

    #[test]
    fn secret_types_are_exhaustive() {
        assert_eq!(SecretType::ALL.len(), 8);
        assert!(SecretType::ALL.contains(&SecretType::ApiKey));
        assert!(SecretType::ALL.contains(&SecretType::Blob));
    }

    #[test]
    fn secret_type_serde_uses_spec_names() {
        let encoded = serde_json::to_string(&SecretType::OAuthApp).expect("serialize");
        assert_eq!(encoded, "\"OAUTH_APP\"");
        let decoded: SecretType = serde_json::from_str("\"SSH_KEY\"").expect("deserialize");
        assert_eq!(decoded, SecretType::SshKey);
    }
}
