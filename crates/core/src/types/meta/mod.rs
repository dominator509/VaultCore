use serde::{Deserialize, Serialize};

use crate::{
    error::VaultError,
    validation::{
        validate_labels, validate_optional_field, validate_required_field, validate_uri_list,
        FieldRule,
    },
};

const TEXT_FIELD_BYTES: usize = 255;
const NOTE_BYTES: usize = 4_096;

pub trait ValidateMeta {
    /// Validate this metadata value against its type-specific schema.
    ///
    /// # Errors
    ///
    /// Returns `VaultErrorCode::ValidationInvalidField` or
    /// `VaultErrorCode::ValidationInvalidLabel` when any required metadata field or
    /// shared label rule is violated.
    fn validate(&self) -> Result<(), VaultError>;
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ApiKeyMeta {
    pub service: String,
    pub key_name: String,
    pub environment: Option<String>,
    pub labels: Vec<String>,
}

impl ValidateMeta for ApiKeyMeta {
    fn validate(&self) -> Result<(), VaultError> {
        validate_required_field(
            FieldRule::required("service", TEXT_FIELD_BYTES),
            &self.service,
        )?;
        validate_required_field(
            FieldRule::required("key_name", TEXT_FIELD_BYTES),
            &self.key_name,
        )?;
        validate_optional_field(
            FieldRule::optional("environment", TEXT_FIELD_BYTES),
            self.environment.as_deref(),
        )?;
        validate_labels(&self.labels)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LoginMeta {
    pub service: String,
    pub username: String,
    pub url: Option<String>,
    pub labels: Vec<String>,
}

impl ValidateMeta for LoginMeta {
    fn validate(&self) -> Result<(), VaultError> {
        validate_required_field(
            FieldRule::required("service", TEXT_FIELD_BYTES),
            &self.service,
        )?;
        validate_required_field(
            FieldRule::required("username", TEXT_FIELD_BYTES),
            &self.username,
        )?;
        validate_optional_field(FieldRule::optional("url", 2_048), self.url.as_deref())?;
        validate_labels(&self.labels)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OAuthAppMeta {
    pub provider: String,
    pub client_id: String,
    pub redirect_uris: Vec<String>,
    pub labels: Vec<String>,
}

impl ValidateMeta for OAuthAppMeta {
    fn validate(&self) -> Result<(), VaultError> {
        validate_required_field(
            FieldRule::required("provider", TEXT_FIELD_BYTES),
            &self.provider,
        )?;
        validate_required_field(
            FieldRule::required("client_id", TEXT_FIELD_BYTES),
            &self.client_id,
        )?;
        validate_uri_list("redirect_uris", &self.redirect_uris)?;
        validate_labels(&self.labels)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SshKeyMeta {
    pub key_name: String,
    pub algorithm: String,
    pub public_fingerprint: String,
    pub labels: Vec<String>,
}

impl ValidateMeta for SshKeyMeta {
    fn validate(&self) -> Result<(), VaultError> {
        validate_required_field(
            FieldRule::required("key_name", TEXT_FIELD_BYTES),
            &self.key_name,
        )?;
        validate_required_field(
            FieldRule::required("algorithm", TEXT_FIELD_BYTES),
            &self.algorithm,
        )?;
        validate_required_field(
            FieldRule::required("public_fingerprint", TEXT_FIELD_BYTES),
            &self.public_fingerprint,
        )?;
        validate_labels(&self.labels)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WalletKeyMeta {
    pub wallet_name: String,
    pub network: String,
    pub public_address: Option<String>,
    pub labels: Vec<String>,
}

impl ValidateMeta for WalletKeyMeta {
    fn validate(&self) -> Result<(), VaultError> {
        validate_required_field(
            FieldRule::required("wallet_name", TEXT_FIELD_BYTES),
            &self.wallet_name,
        )?;
        validate_required_field(
            FieldRule::required("network", TEXT_FIELD_BYTES),
            &self.network,
        )?;
        validate_optional_field(
            FieldRule::optional("public_address", TEXT_FIELD_BYTES),
            self.public_address.as_deref(),
        )?;
        validate_labels(&self.labels)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CertMeta {
    pub common_name: String,
    pub issuer: Option<String>,
    pub serial_number: Option<String>,
    pub labels: Vec<String>,
}

impl ValidateMeta for CertMeta {
    fn validate(&self) -> Result<(), VaultError> {
        validate_required_field(
            FieldRule::required("common_name", TEXT_FIELD_BYTES),
            &self.common_name,
        )?;
        validate_optional_field(
            FieldRule::optional("issuer", TEXT_FIELD_BYTES),
            self.issuer.as_deref(),
        )?;
        validate_optional_field(
            FieldRule::optional("serial_number", TEXT_FIELD_BYTES),
            self.serial_number.as_deref(),
        )?;
        validate_labels(&self.labels)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NoteMeta {
    pub title: String,
    pub summary: Option<String>,
    pub labels: Vec<String>,
}

impl ValidateMeta for NoteMeta {
    fn validate(&self) -> Result<(), VaultError> {
        validate_required_field(FieldRule::required("title", TEXT_FIELD_BYTES), &self.title)?;
        validate_optional_field(
            FieldRule::optional("summary", NOTE_BYTES),
            self.summary.as_deref(),
        )?;
        validate_labels(&self.labels)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BlobMeta {
    pub file_name: String,
    pub media_type: Option<String>,
    pub size_bytes: u64,
    pub labels: Vec<String>,
}

impl ValidateMeta for BlobMeta {
    fn validate(&self) -> Result<(), VaultError> {
        validate_required_field(
            FieldRule::required("file_name", TEXT_FIELD_BYTES),
            &self.file_name,
        )?;
        validate_optional_field(
            FieldRule::optional("media_type", TEXT_FIELD_BYTES),
            self.media_type.as_deref(),
        )?;
        validate_labels(&self.labels)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "meta")]
pub enum SecretMeta {
    #[serde(rename = "API_KEY")]
    ApiKey(ApiKeyMeta),
    #[serde(rename = "LOGIN")]
    Login(LoginMeta),
    #[serde(rename = "OAUTH_APP")]
    OAuthApp(OAuthAppMeta),
    #[serde(rename = "SSH_KEY")]
    SshKey(SshKeyMeta),
    #[serde(rename = "WALLET_KEY")]
    WalletKey(WalletKeyMeta),
    #[serde(rename = "CERT")]
    Cert(CertMeta),
    #[serde(rename = "NOTE")]
    Note(NoteMeta),
    #[serde(rename = "BLOB")]
    Blob(BlobMeta),
}

impl ValidateMeta for SecretMeta {
    fn validate(&self) -> Result<(), VaultError> {
        match self {
            Self::ApiKey(meta) => meta.validate(),
            Self::Login(meta) => meta.validate(),
            Self::OAuthApp(meta) => meta.validate(),
            Self::SshKey(meta) => meta.validate(),
            Self::WalletKey(meta) => meta.validate(),
            Self::Cert(meta) => meta.validate(),
            Self::Note(meta) => meta.validate(),
            Self::Blob(meta) => meta.validate(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        ApiKeyMeta, BlobMeta, CertMeta, LoginMeta, NoteMeta, OAuthAppMeta, SecretMeta, SshKeyMeta,
        ValidateMeta, WalletKeyMeta,
    };

    #[test]
    fn metadata_validators_accept_minimum_valid_shapes() {
        let labels = vec!["prod".to_owned()];
        ApiKeyMeta {
            service: "github".to_owned(),
            key_name: "ci".to_owned(),
            environment: None,
            labels: labels.clone(),
        }
        .validate()
        .expect("valid api key meta");
        LoginMeta {
            service: "console".to_owned(),
            username: "owner".to_owned(),
            url: None,
            labels: labels.clone(),
        }
        .validate()
        .expect("valid login meta");
        OAuthAppMeta {
            provider: "oauth".to_owned(),
            client_id: "client".to_owned(),
            redirect_uris: vec!["https://example.test/callback".to_owned()],
            labels: labels.clone(),
        }
        .validate()
        .expect("valid oauth meta");
        SshKeyMeta {
            key_name: "deploy".to_owned(),
            algorithm: "ed25519".to_owned(),
            public_fingerprint: "SHA256:abc".to_owned(),
            labels: labels.clone(),
        }
        .validate()
        .expect("valid ssh key meta");
        WalletKeyMeta {
            wallet_name: "treasury".to_owned(),
            network: "mainnet".to_owned(),
            public_address: None,
            labels: labels.clone(),
        }
        .validate()
        .expect("valid wallet meta");
        CertMeta {
            common_name: "example.test".to_owned(),
            issuer: None,
            serial_number: None,
            labels: labels.clone(),
        }
        .validate()
        .expect("valid cert meta");
        NoteMeta {
            title: "runbook".to_owned(),
            summary: None,
            labels: labels.clone(),
        }
        .validate()
        .expect("valid note meta");
        BlobMeta {
            file_name: "archive.bin".to_owned(),
            media_type: None,
            size_bytes: 0,
            labels,
        }
        .validate()
        .expect("valid blob meta");
    }

    #[test]
    fn metadata_validators_reject_required_field_gaps() {
        let error = ApiKeyMeta {
            service: String::new(),
            key_name: "ci".to_owned(),
            environment: None,
            labels: Vec::new(),
        }
        .validate()
        .expect_err("missing service");
        assert_eq!(error.field.as_deref(), Some("service"));
    }

    #[test]
    fn oauth_redirect_uris_require_schemes() {
        let error = OAuthAppMeta {
            provider: "oauth".to_owned(),
            client_id: "client".to_owned(),
            redirect_uris: vec!["example.test/callback".to_owned()],
            labels: Vec::new(),
        }
        .validate()
        .expect_err("missing scheme");
        assert_eq!(error.field.as_deref(), Some("redirect_uris"));
    }

    #[test]
    fn secret_meta_round_trips_through_serde() {
        let meta = SecretMeta::Note(NoteMeta {
            title: "release".to_owned(),
            summary: Some("handoff".to_owned()),
            labels: vec!["ops".to_owned()],
        });
        let encoded = serde_json::to_string(&meta).expect("serialize");
        let decoded: SecretMeta = serde_json::from_str(&encoded).expect("deserialize");
        assert_eq!(decoded, meta);
    }
}
