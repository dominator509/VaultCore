use serde_json::{Map, Value};
use vaultcore_core::obs::redact_event;

#[must_use]
pub fn emit_builder_log(fields: &Map<String, Value>) -> String {
    redact_event(fields).to_json_line()
}

#[cfg(test)]
mod tests {
    use serde_json::{json, Map};

    use super::emit_builder_log;

    #[test]
    fn obs_builder_log_uses_core_redaction() {
        let mut event = Map::new();
        event.insert("component".to_owned(), json!("builder"));
        event.insert("op".to_owned(), json!("unlock"));
        event.insert("payload".to_owned(), json!("VC_SYNTHETIC_SECRET_PAYLOAD"));

        let line = emit_builder_log(&event);

        assert!(line.contains("builder"));
        assert!(line.contains("[REDACTED]"));
        assert!(!line.contains("VC_SYNTHETIC_SECRET_PAYLOAD"));
    }
}
