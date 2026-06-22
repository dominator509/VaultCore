use serde::{Deserialize, Serialize};
use std::{fmt, str::FromStr};
use ulid::Ulid;

use crate::error::{VaultError, VaultErrorCode};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SecretId(Ulid);

impl SecretId {
    #[must_use]
    pub fn generate() -> Self {
        Self(Ulid::new())
    }

    #[must_use]
    pub const fn from_ulid(value: Ulid) -> Self {
        Self(value)
    }

    #[must_use]
    pub const fn as_ulid(self) -> Ulid {
        self.0
    }
}

impl fmt::Display for SecretId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.0)
    }
}

impl FromStr for SecretId {
    type Err = VaultError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ulid::from_string(value).map(Self).map_err(|_| {
            VaultError::new(
                VaultErrorCode::ValidationInvalidField,
                Some("id".to_owned()),
                "id must be a valid ULID",
            )
        })
    }
}

#[cfg(test)]
mod tests {
    use super::SecretId;
    use std::str::FromStr;

    #[test]
    fn secret_id_round_trips_as_ulid() {
        let id = SecretId::generate();
        let parsed = SecretId::from_str(&id.to_string()).expect("parse ULID");
        assert_eq!(parsed, id);
    }

    #[test]
    fn invalid_secret_id_is_rejected() {
        let error = SecretId::from_str("not-a-ulid").expect_err("invalid ULID");
        assert_eq!(error.field.as_deref(), Some("id"));
    }
}
