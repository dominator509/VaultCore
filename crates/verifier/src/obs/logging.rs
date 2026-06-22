use serde_json::{Map, Value};
use vaultcore_core::obs::redact_event;

#[must_use]
pub fn emit_verifier_log(fields: &Map<String, Value>) -> String {
    redact_event(fields).to_json_line()
}

#[cfg(test)]
mod tests {
    use serde_json::{json, Map};

    use super::emit_verifier_log;

    #[test]
    fn obs_verifier_log_uses_core_redaction() {
        let mut event = Map::new();
        event.insert("component".to_owned(), json!("verifier"));
        event.insert("op".to_owned(), json!("countersign"));
        event.insert("signing_key".to_owned(), json!("VC_SYNTHETIC_SIGNING_KEY"));

        let line = emit_verifier_log(&event);

        assert!(line.contains("verifier"));
        assert!(line.contains("[REDACTED]"));
        assert!(!line.contains("VC_SYNTHETIC_SIGNING_KEY"));
    }
}
