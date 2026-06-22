use serde_json::{Map, Value};

pub const ALLOWED_LOG_FIELDS: &[&str] = &[
    "ts",
    "level",
    "component",
    "op",
    "session_id",
    "secret_id",
    "role",
    "status",
    "duration_ms",
    "audit_seq",
    "err_code",
];

pub const FORBIDDEN_LOG_FIELDS: &[&str] = &[
    "payload",
    "payload_envelope",
    "payload_handle",
    "signing_key",
    "verification_key_secret",
    "derived_key",
    "nonce",
    "passphrase",
    "biometric_template",
    "audit_preimage",
    "vault_path",
    "file_path",
    "raw_secret",
    "plaintext",
];

pub const SYNTHETIC_SECRET_MARKERS: &[&str] = &[
    "VC_SYNTHETIC_SECRET_PAYLOAD",
    "VC_SYNTHETIC_SIGNING_KEY",
    "VC_SYNTHETIC_DERIVED_KEY",
    "VC_SYNTHETIC_NONCE",
    "VC_SYNTHETIC_PASSPHRASE",
    "VC_SYNTHETIC_BIOMETRIC_TEMPLATE",
    "VC_SYNTHETIC_AUDIT_PREIMAGE",
    "VC_SYNTHETIC_VAULT_PATH",
];

const REDACTED: &str = "[REDACTED]";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RedactedLogEvent {
    fields: Map<String, Value>,
}

impl RedactedLogEvent {
    #[must_use]
    pub fn as_value(&self) -> Value {
        Value::Object(self.fields.clone())
    }

    /// Serializes the redacted log event as one JSON line.
    ///
    /// # Panics
    ///
    /// Panics only if serde cannot serialize a `serde_json::Value` map.
    #[must_use]
    pub fn to_json_line(&self) -> String {
        serde_json::to_string(&self.fields).expect("redacted log event must serialize")
    }
}

#[must_use]
pub fn redact_event(input: &Map<String, Value>) -> RedactedLogEvent {
    let fields = input
        .iter()
        .map(|(field, value)| {
            let redacted = if is_allowed_field(field) && !is_forbidden_field(field) {
                redact_value(value)
            } else {
                Value::String(REDACTED.to_owned())
            };
            (field.clone(), redacted)
        })
        .collect();

    RedactedLogEvent { fields }
}

pub fn redact_value(value: &Value) -> Value {
    match value {
        Value::String(text) if contains_secret_marker(text) => Value::String(REDACTED.to_owned()),
        Value::String(_) | Value::Number(_) | Value::Bool(_) | Value::Null => value.clone(),
        Value::Array(items) => Value::Array(items.iter().map(redact_value).collect()),
        Value::Object(fields) => Value::Object(
            fields
                .iter()
                .map(|(field, value)| {
                    let redacted = if is_allowed_field(field) && !is_forbidden_field(field) {
                        redact_value(value)
                    } else {
                        Value::String(REDACTED.to_owned())
                    };
                    (field.clone(), redacted)
                })
                .collect(),
        ),
    }
}

fn is_allowed_field(field: &str) -> bool {
    ALLOWED_LOG_FIELDS
        .iter()
        .any(|allowed| field.eq_ignore_ascii_case(allowed))
}

fn is_forbidden_field(field: &str) -> bool {
    let normalized = field.to_ascii_lowercase();
    FORBIDDEN_LOG_FIELDS
        .iter()
        .any(|forbidden| normalized.contains(forbidden))
}

fn contains_secret_marker(text: &str) -> bool {
    SYNTHETIC_SECRET_MARKERS
        .iter()
        .any(|marker| text.contains(marker))
}

#[cfg(test)]
mod tests {
    use serde_json::{json, Map, Value};

    use super::{redact_event, redact_value, SYNTHETIC_SECRET_MARKERS};

    #[test]
    fn obs_redaction_removes_forbidden_and_unknown_fields() {
        let mut event = Map::new();
        event.insert("ts".to_owned(), json!("2026-06-22T12:00:00Z"));
        event.insert("level".to_owned(), json!("info"));
        event.insert("component".to_owned(), json!("builder"));
        event.insert("op".to_owned(), json!("reveal"));
        event.insert("session_id".to_owned(), json!("session-1"));
        event.insert("payload".to_owned(), json!("VC_SYNTHETIC_SECRET_PAYLOAD"));
        event.insert("custom_debug".to_owned(), json!("VC_SYNTHETIC_SIGNING_KEY"));

        let line = redact_event(&event).to_json_line();

        assert!(line.contains("session-1"));
        assert!(line.contains("[REDACTED]"));
        for marker in SYNTHETIC_SECRET_MARKERS {
            assert!(!line.contains(marker), "marker leaked: {marker}");
        }
    }

    #[test]
    fn obs_redaction_scrubs_markers_inside_allowed_values() {
        let event = json!({
            "status": "error VC_SYNTHETIC_DERIVED_KEY",
            "err_code": "IPC_ERROR",
            "duration_ms": 42
        });
        let Value::Object(fields) = event else {
            panic!("event object");
        };

        let line = redact_event(&fields).to_json_line();

        assert!(line.contains("IPC_ERROR"));
        assert!(line.contains("42"));
        assert!(!line.contains("VC_SYNTHETIC_DERIVED_KEY"));
    }

    #[test]
    fn obs_redaction_scrubs_nested_unknown_fields() {
        let value = json!({
            "op": "unlock",
            "details": {
                "payload": "VC_SYNTHETIC_SECRET_PAYLOAD",
                "session_id": "session-2"
            },
            "audit_seq": 7
        });

        let redacted = redact_value(&value).to_string();

        assert!(redacted.contains("[REDACTED]"));
        assert!(!redacted.contains("VC_SYNTHETIC_SECRET_PAYLOAD"));
        assert!(!redacted.contains("session-2"));
    }
}
