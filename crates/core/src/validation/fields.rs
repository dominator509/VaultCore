use crate::error::VaultError;

pub const MAX_NAME_BYTES: usize = 255;
pub const MAX_LABELS: usize = 32;
pub const MAX_LABEL_BYTES: usize = 64;
pub const MAX_URI_BYTES: usize = 2_048;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FieldRule {
    pub field: &'static str,
    pub max_bytes: usize,
    pub required: bool,
}

impl FieldRule {
    #[must_use]
    pub const fn required(field: &'static str, max_bytes: usize) -> Self {
        Self {
            field,
            max_bytes,
            required: true,
        }
    }

    #[must_use]
    pub const fn optional(field: &'static str, max_bytes: usize) -> Self {
        Self {
            field,
            max_bytes,
            required: false,
        }
    }
}

/// Validate a user-visible secret name.
///
/// # Errors
///
/// Returns `VaultErrorCode::ValidationInvalidName` when the name is empty, longer than
/// 255 UTF-8 bytes, or contains a control character.
pub fn validate_name(name: &str) -> Result<(), VaultError> {
    if name.is_empty() {
        return Err(VaultError::invalid_name("name is required"));
    }
    if name.len() > MAX_NAME_BYTES {
        return Err(VaultError::invalid_name("name exceeds 255 UTF-8 bytes"));
    }
    if contains_control_character(name) {
        return Err(VaultError::invalid_name(
            "name must not contain control characters",
        ));
    }
    Ok(())
}

/// Validate shared metadata labels.
///
/// # Errors
///
/// Returns `VaultErrorCode::ValidationInvalidLabel` when there are more than 32 labels,
/// when a label is empty, longer than 64 UTF-8 bytes, or contains a control character.
pub fn validate_labels(labels: &[String]) -> Result<(), VaultError> {
    if labels.len() > MAX_LABELS {
        return Err(VaultError::invalid_label("labels exceed 32 entries"));
    }

    for label in labels {
        if label.is_empty() {
            return Err(VaultError::invalid_label("labels must not be empty"));
        }
        if label.len() > MAX_LABEL_BYTES {
            return Err(VaultError::invalid_label("label exceeds 64 UTF-8 bytes"));
        }
        if contains_control_character(label) {
            return Err(VaultError::invalid_label(
                "labels must not contain control characters",
            ));
        }
    }

    Ok(())
}

/// Validate a required metadata field.
///
/// # Errors
///
/// Returns `VaultErrorCode::ValidationInvalidField` when the field is empty, too long,
/// or contains a control character.
pub fn validate_required_field(rule: FieldRule, value: &str) -> Result<(), VaultError> {
    debug_assert!(rule.required);
    validate_field(rule, value)
}

/// Validate an optional metadata field when present.
///
/// # Errors
///
/// Returns `VaultErrorCode::ValidationInvalidField` when the provided field is too long
/// or contains a control character.
pub fn validate_optional_field(rule: FieldRule, value: Option<&str>) -> Result<(), VaultError> {
    debug_assert!(!rule.required);
    if let Some(value) = value {
        validate_field(rule, value)?;
    }
    Ok(())
}

/// Validate a non-empty list of URI-like metadata fields.
///
/// # Errors
///
/// Returns `VaultErrorCode::ValidationInvalidField` when the list is empty, a URI is too
/// long, contains a control character, or lacks a scheme separator.
pub fn validate_uri_list(field: &'static str, values: &[String]) -> Result<(), VaultError> {
    if values.is_empty() {
        return Err(VaultError::invalid_field(
            field,
            "at least one URI is required",
        ));
    }

    for value in values {
        validate_required_field(FieldRule::required(field, MAX_URI_BYTES), value)?;
        if !value.contains(':') {
            return Err(VaultError::invalid_field(
                field,
                "URI must include a scheme",
            ));
        }
    }

    Ok(())
}

fn validate_field(rule: FieldRule, value: &str) -> Result<(), VaultError> {
    if rule.required && value.is_empty() {
        return Err(VaultError::invalid_field(rule.field, "field is required"));
    }
    if value.len() > rule.max_bytes {
        return Err(VaultError::invalid_field(
            rule.field,
            format!("field exceeds {} UTF-8 bytes", rule.max_bytes),
        ));
    }
    if contains_control_character(value) {
        return Err(VaultError::invalid_field(
            rule.field,
            "field must not contain control characters",
        ));
    }
    Ok(())
}

fn contains_control_character(value: &str) -> bool {
    value.chars().any(char::is_control)
}

#[cfg(test)]
mod tests {
    use super::{validate_labels, validate_name};

    #[test]
    fn name_validation_accepts_normal_names() {
        validate_name("production-api").expect("valid name");
    }

    #[test]
    fn name_validation_rejects_empty_long_and_control_values() {
        assert!(validate_name("").is_err());
        assert!(validate_name(&"a".repeat(256)).is_err());
        assert!(validate_name("bad\nname").is_err());
    }

    #[test]
    fn label_validation_enforces_limits() {
        validate_labels(&["prod".to_owned(), "shared".to_owned()]).expect("valid labels");
        assert!(validate_labels(&vec!["x".to_owned(); 33]).is_err());
        assert!(validate_labels(&["a".repeat(65)]).is_err());
        assert!(validate_labels(&["bad\tlabel".to_owned()]).is_err());
    }
}
