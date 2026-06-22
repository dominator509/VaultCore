pub const METRICS_PATH: &str = "/metrics";
pub const BUILDER_METRICS: &[&str] = &[
    "builder.unlock.success_total",
    "builder.unlock.failure_total",
    "builder.reveal.success_total",
    "builder.reveal.duration_ms",
    "builder.write.success_total",
    "ipc.signature_failures_total",
    "ipc.replay_rejections_total",
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MetricSample {
    pub name: &'static str,
    pub value: u64,
}

#[must_use]
pub fn builder_metrics_snapshot() -> Vec<MetricSample> {
    BUILDER_METRICS
        .iter()
        .map(|name| MetricSample { name, value: 0 })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{builder_metrics_snapshot, BUILDER_METRICS, METRICS_PATH};

    #[test]
    fn obs_builder_metrics_endpoint_contract() {
        let metrics = builder_metrics_snapshot();

        assert_eq!(METRICS_PATH, "/metrics");
        for required in BUILDER_METRICS {
            assert!(metrics.iter().any(|metric| metric.name == *required));
        }
    }
}
