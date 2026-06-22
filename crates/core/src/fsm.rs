use crate::{
    error::VaultError,
    types::{
        LifecycleState,
        LifecycleState::{
            Active, Archived, Draft, Expired, ExpiringSoon, Purged, Rotating, SoftDeleted,
        },
    },
};

pub const LEGAL_TRANSITIONS: &[(LifecycleState, LifecycleState)] = &[
    (Draft, Active),
    (Active, ExpiringSoon),
    (ExpiringSoon, Expired),
    (Expired, Rotating),
    (Rotating, Active),
    (Active, Archived),
    (Archived, Active),
    (Archived, SoftDeleted),
    (SoftDeleted, Archived),
    (SoftDeleted, Purged),
];

/// Validate a lifecycle state transition.
///
/// # Errors
///
/// Returns `VaultErrorCode::FsmInvalidTransition` when the requested transition is not
/// present in the EP-002/SPEC-001 transition table.
pub fn transition(from: LifecycleState, to: LifecycleState) -> Result<(), VaultError> {
    if can_transition(from, to) {
        Ok(())
    } else {
        Err(VaultError::fsm_invalid_transition(from, to))
    }
}

#[must_use]
pub fn can_transition(from: LifecycleState, to: LifecycleState) -> bool {
    LEGAL_TRANSITIONS
        .iter()
        .any(|&(candidate_from, candidate_to)| candidate_from == from && candidate_to == to)
}

#[cfg(test)]
mod tests {
    use super::{can_transition, transition, LEGAL_TRANSITIONS};
    use crate::types::LifecycleState;

    #[test]
    fn every_declared_transition_is_legal() {
        for &(from, to) in LEGAL_TRANSITIONS {
            transition(from, to).expect("declared transition is legal");
        }
    }

    #[test]
    fn illegal_transitions_return_fsm_error() {
        let error = transition(LifecycleState::Draft, LifecycleState::Purged)
            .expect_err("draft cannot purge directly");
        assert_eq!(error.code.as_str(), "VC-FSM-001");
    }

    #[test]
    fn purged_is_terminal() {
        for to in LifecycleState::ALL {
            assert!(!can_transition(LifecycleState::Purged, to));
        }
    }

    #[test]
    fn transition_table_covers_every_state_pair() {
        for from in LifecycleState::ALL {
            for to in LifecycleState::ALL {
                let result = transition(from, to);
                if can_transition(from, to) {
                    assert!(result.is_ok(), "{from} -> {to} should be legal");
                } else {
                    assert!(result.is_err(), "{from} -> {to} should be illegal");
                }
            }
        }
    }

    #[test]
    fn archived_and_soft_deleted_are_reversible() {
        assert!(can_transition(
            LifecycleState::Archived,
            LifecycleState::Active
        ));
        assert!(can_transition(
            LifecycleState::SoftDeleted,
            LifecycleState::Archived
        ));
    }
}
