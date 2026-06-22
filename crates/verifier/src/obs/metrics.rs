pub const METRICS_PATH: &str = "/metrics";
pub const VERIFIER_METRICS: &[&str] = &[
    "verifier.countersign.success_total",
    "verifier.countersign.denied_total",
    "verifier.audit.append_total",
    "ipc.signature_failures_total",
    "ipc.replay_rejections_total",
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MetricSample {
    pub name: &'static str,
    pub value: u64,
}

#[must_use]
pub fn verifier_metrics_snapshot() -> Vec<MetricSample> {
    VERIFIER_METRICS
        .iter()
        .map(|name| MetricSample { name, value: 0 })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{verifier_metrics_snapshot, METRICS_PATH, VERIFIER_METRICS};

    #[test]
    fn obs_verifier_metrics_endpoint_contract() {
        let metrics = verifier_metrics_snapshot();

        assert_eq!(METRICS_PATH, "/metrics");
        for required in VERIFIER_METRICS {
            assert!(metrics.iter().any(|metric| metric.name == *required));
        }
    }
}
