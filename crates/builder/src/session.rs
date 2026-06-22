use serde::{Deserialize, Serialize};

pub const DEFAULT_IDLE_TIMEOUT_SECS: u64 = 15 * 60;
pub const DEFAULT_ABSOLUTE_TIMEOUT_SECS: u64 = 8 * 60 * 60;
pub const DEFAULT_LOCKOUT_BASE_SECS: u64 = 1;
pub const DEFAULT_LOCKOUT_MAX_SECS: u64 = 5 * 60;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SessionToken {
    pub session_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuthProof {
    pub method: String,
    pub proof: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SessionPolicy {
    pub idle_timeout_secs: u64,
    pub absolute_timeout_secs: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SessionState {
    pub issued_at_secs: u64,
    pub last_seen_secs: u64,
    pub revoked: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LockoutPolicy {
    pub base_delay_secs: u64,
    pub max_delay_secs: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct LockoutState {
    pub failure_count: u32,
    pub locked_until_secs: u64,
}

impl Default for SessionPolicy {
    fn default() -> Self {
        Self {
            idle_timeout_secs: DEFAULT_IDLE_TIMEOUT_SECS,
            absolute_timeout_secs: DEFAULT_ABSOLUTE_TIMEOUT_SECS,
        }
    }
}

impl SessionState {
    #[must_use]
    pub const fn issue(now_secs: u64) -> Self {
        Self {
            issued_at_secs: now_secs,
            last_seen_secs: now_secs,
            revoked: false,
        }
    }

    #[must_use]
    pub fn is_expired(self, policy: SessionPolicy, now_secs: u64) -> bool {
        self.revoked
            || elapsed(now_secs, self.last_seen_secs) >= policy.idle_timeout_secs
            || elapsed(now_secs, self.issued_at_secs) >= policy.absolute_timeout_secs
    }

    #[must_use]
    pub fn touch(self, policy: SessionPolicy, now_secs: u64) -> Option<Self> {
        if self.is_expired(policy, now_secs) {
            None
        } else {
            Some(Self {
                last_seen_secs: now_secs,
                ..self
            })
        }
    }

    #[must_use]
    pub const fn revoke(self) -> Self {
        Self {
            revoked: true,
            ..self
        }
    }
}

impl Default for LockoutPolicy {
    fn default() -> Self {
        Self {
            base_delay_secs: DEFAULT_LOCKOUT_BASE_SECS,
            max_delay_secs: DEFAULT_LOCKOUT_MAX_SECS,
        }
    }
}

impl LockoutState {
    #[must_use]
    pub fn is_locked_out(self, now_secs: u64) -> bool {
        now_secs < self.locked_until_secs
    }

    #[must_use]
    pub fn register_failure(self, policy: LockoutPolicy, now_secs: u64) -> Self {
        let failure_count = self.failure_count.saturating_add(1);
        let exponent = failure_count.saturating_sub(1).min(63);
        let multiplier = 1u64.checked_shl(exponent).unwrap_or(u64::MAX);
        let delay = policy
            .base_delay_secs
            .saturating_mul(multiplier)
            .min(policy.max_delay_secs);

        Self {
            failure_count,
            locked_until_secs: now_secs.saturating_add(delay),
        }
    }

    #[must_use]
    pub const fn register_success(self) -> Self {
        Self {
            failure_count: 0,
            locked_until_secs: 0,
        }
    }
}

const fn elapsed(now_secs: u64, then_secs: u64) -> u64 {
    now_secs.saturating_sub(then_secs)
}

#[cfg(test)]
mod tests {
    use super::{
        LockoutPolicy, LockoutState, SessionPolicy, SessionState, DEFAULT_ABSOLUTE_TIMEOUT_SECS,
        DEFAULT_IDLE_TIMEOUT_SECS,
    };

    #[test]
    fn auth_session_idle_timeout_expires_without_activity() {
        let policy = SessionPolicy::default();
        let session = SessionState::issue(100);

        assert!(!session.is_expired(policy, 100 + DEFAULT_IDLE_TIMEOUT_SECS - 1));
        assert!(session.is_expired(policy, 100 + DEFAULT_IDLE_TIMEOUT_SECS));
    }

    #[test]
    fn auth_session_touch_extends_idle_but_not_absolute_timeout() {
        let policy = SessionPolicy::default();
        let session = SessionState::issue(100)
            .touch(policy, 100 + DEFAULT_IDLE_TIMEOUT_SECS - 1)
            .expect("active session");

        assert!(!session.is_expired(policy, 100 + DEFAULT_IDLE_TIMEOUT_SECS + 10));
        assert!(session.is_expired(policy, 100 + DEFAULT_ABSOLUTE_TIMEOUT_SECS));
    }

    #[test]
    fn auth_session_lock_revokes_immediately() {
        let policy = SessionPolicy::default();
        let session = SessionState::issue(100).revoke();

        assert!(session.is_expired(policy, 101));
        assert!(session.touch(policy, 101).is_none());
    }

    #[test]
    fn auth_lockout_uses_exponential_backoff_per_device() {
        let policy = LockoutPolicy {
            base_delay_secs: 2,
            max_delay_secs: 20,
        };
        let state = LockoutState::default()
            .register_failure(policy, 100)
            .register_failure(policy, 100)
            .register_failure(policy, 100)
            .register_failure(policy, 100);

        assert_eq!(state.failure_count, 4);
        assert_eq!(state.locked_until_secs, 116);
        assert!(state.is_locked_out(115));
        assert!(!state.is_locked_out(116));
    }

    #[test]
    fn auth_lockout_caps_delay_and_resets_only_on_success() {
        let policy = LockoutPolicy {
            base_delay_secs: 10,
            max_delay_secs: 30,
        };
        let failed = LockoutState::default()
            .register_failure(policy, 0)
            .register_failure(policy, 0)
            .register_failure(policy, 0)
            .register_failure(policy, 0);

        assert_eq!(failed.locked_until_secs, 30);
        assert_eq!(failed.register_success().failure_count, 0);
    }
}
