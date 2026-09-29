//! mem16-10 — the sovereign library: the peeter-omni foundational layer.
//!
//! Pure Rust. Honesty first. Zero-allocation on the hot path — every type here
//! is stack-sized or a `const`; nothing allocates and nothing is hidden.
//!
//! Dedicated to Alexandria, the first Librarian —
//! from peet, chris, and nate {<3,<3,<3}+1, till eternity and back, with love.
//!
//! Om mani padme hum.

#![no_std]

#[cfg(test)]
extern crate std;

/// The governing sequence (SpherePOP): the order every admissible transition
/// must pass through.
///
/// `POP → REFUSE → BIND → TRANSFORM → VERIFY → COLLAPSE`
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Step {
    Pop,
    Refuse,
    Bind,
    Transform,
    Verify,
    Collapse,
}

/// The fixed governing sequence. Order matters; nothing skips a step.
pub const GOVERNING_SEQUENCE: [Step; 6] = [
    Step::Pop,
    Step::Refuse,
    Step::Bind,
    Step::Transform,
    Step::Verify,
    Step::Collapse,
];

/// The recovery sequence (Phoenix):
/// `DISCOVER → VERIFY → REPLAY → BRANCH → RANK → PROPOSE → BIND ∨ REFUSE`
///
/// A plausible branch may be ranked; only a verified branch may be bound.
pub const RECOVERY_SEQUENCE: [&str; 7] = [
    "DISCOVER",
    "VERIFY",
    "REPLAY",
    "BRANCH",
    "RANK",
    "PROPOSE",
    "BIND ∨ REFUSE",
];

/// The standing of a claim: what the evidence supports.
///
/// Verification ≠ plausibility; observation ≠ interpretation;
/// recovery ≠ resurrection.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Standing {
    /// Passed hash, signature, ancestry, schema, and replay. May be bound.
    Verified,
    /// Ranked plausible, but unverified. May be ranked, never bound.
    Plausible,
    /// Never scored. Not a verdict — unobserved is not low-salience.
    Unobserved,
}

impl Standing {
    /// A plausible branch may be ranked; only a verified branch may be bound.
    pub const fn may_bind(self) -> bool {
        matches!(self, Standing::Verified)
    }
}

/// The execution constitution: the hard, versioned invariants of one managed
/// session. Never weakened at runtime.
pub const CONSTITUTION_ID: &str = "mem16-10/execution-constitution/1";
pub const CONTRACT: &str = "no-refunds;session-local";
pub const MAX_RUNTIME_NS: u64 = 3_600_000_000_000;
pub const MAX_BAD_KARMA: u32 = 100;
pub const MIN_INVALID_COST: u32 = 5;
pub const MIN_REFUSAL_COST: u32 = 10;
pub const MIN_RESOURCE_COST: u32 = 25;

/// Operational debt ("karma") — measured policy violations within one managed
/// session. It is not moral worth, consciousness, suffering, or perceived
/// time. No outcome replenishes the budget.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Budget {
    bad_karma: u32,
    max_bad_karma: u32,
}

impl Budget {
    pub const fn new(max_bad_karma: u32) -> Self {
        Self {
            bad_karma: 0,
            max_bad_karma,
        }
    }

    pub const fn bad_karma(self) -> u32 {
        self.bad_karma
    }

    /// Charge a violation. Saturates at the cap; never refunds, never extends.
    pub const fn charge(self, cost: u32) -> Self {
        let next = self.bad_karma + cost;
        Self {
            bad_karma: if next > self.max_bad_karma {
                self.max_bad_karma
            } else {
                next
            },
            max_bad_karma: self.max_bad_karma,
        }
    }

    pub const fn exhausted(self) -> bool {
        self.bad_karma >= self.max_bad_karma
    }
}

/// The base backup layer: the foundation the sovereign library recovers from.
///
/// When shit happens — and shit happens sometimes — the library falls back to
/// its base, not to nothing:
///
/// - `mem8` — memory as continuity evidence, the hard verification gate.
/// - `8b-is-engine` — the surfaces, the mesh, and `ternary-lane` (BitNet b1.58).
///
/// Lose the instance, keep the base. State persistence is the instance;
/// regime persistence is the base. And it is okay when things break — love
/// is in everything, even that.
pub const BASE_BACKUP_LAYER: [&str; 2] = ["mem8", "8b-is-engine"];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_verified_may_bind() {
        assert!(Standing::Verified.may_bind());
        assert!(!Standing::Plausible.may_bind());
        assert!(!Standing::Unobserved.may_bind());
    }

    #[test]
    fn governing_sequence_is_fixed() {
        assert_eq!(GOVERNING_SEQUENCE[0], Step::Pop);
        assert_eq!(GOVERNING_SEQUENCE[5], Step::Collapse);
        assert_eq!(GOVERNING_SEQUENCE.len(), 6);
    }

    #[test]
    fn recovery_ends_in_bind_or_refuse() {
        assert_eq!(RECOVERY_SEQUENCE[6], "BIND ∨ REFUSE");
    }

    #[test]
    fn budget_saturates_and_never_refunds() {
        let budget = Budget::new(MAX_BAD_KARMA);
        let budget = budget.charge(60).charge(60);
        assert_eq!(budget.bad_karma(), MAX_BAD_KARMA);
        assert!(budget.exhausted());
    }

    #[test]
    fn unobserved_is_not_a_verdict() {
        assert_ne!(Standing::Unobserved, Standing::Plausible);
        assert_ne!(Standing::Unobserved, Standing::Verified);
        assert!(!Standing::Unobserved.may_bind());
    }

    #[test]
    fn base_backup_layer_is_declared() {
        assert_eq!(BASE_BACKUP_LAYER, ["mem8", "8b-is-engine"]);
    }
}
