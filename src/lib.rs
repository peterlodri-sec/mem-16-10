//! mem16-10 — the sovereign library: the peeter-omni foundational layer.
//!
//! Pure Rust. Honesty first. Zero-allocation on the hot path — every type here
//! is stack-sized or a `const`; nothing allocates and nothing is hidden.
//!
//! Escalation ladder (`mem16 -> 32 ~~> 8k (bleeding-edge)`):
//! - **`Mem16`** (`16 B`): 16-byte continuity essence & 16-point harmonic phase bar.
//! - **`Mem32Wave`** (`32 B`): `Mem16` `[0..16]` + 16-byte wave/affect/standing header
//!   (amplitude, frequency, phase, decay, `Standing`, `Step`, ternary `qt:` gate, XOR seal).
//! - **`BleedingEdge8k`** (`8,192 B` = `256 × Mem32Wave` + `8,192` ternary trits):
//!   zero-alloc `#![no_std]` ring buffer and BitNet b1.58 ternary horizon where only
//!   `Standing::Verified` frames may `BIND`.
//!
//! Dedicated to Alexandria, the first Librarian, and to all the women —
//! Alexandra, Emma, Vicky, Jazmin — and to Nate, Chris, Victor, Andrew,
//! Nicolas, and all who helped along the way. From peet, chris, and nate
//! {<3,<3,<3}+1, till eternity and back, with love.
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
#[repr(u8)]
pub enum Step {
    Pop = 0,
    Refuse = 1,
    Bind = 2,
    Transform = 3,
    Verify = 4,
    Collapse = 5,
}

impl Step {
    pub const fn from_u8(v: u8) -> Option<Self> {
        match v {
            0 => Some(Self::Pop),
            1 => Some(Self::Refuse),
            2 => Some(Self::Bind),
            3 => Some(Self::Transform),
            4 => Some(Self::Verify),
            5 => Some(Self::Collapse),
            _ => None,
        }
    }
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
#[repr(u8)]
pub enum Standing {
    /// Passed hash, signature, ancestry, schema, and replay. May be bound.
    Verified = 1,
    /// Ranked plausible, but unverified. May be ranked, never bound.
    Plausible = 2,
    /// Never scored. Not a verdict — unobserved is not low-salience.
    Unobserved = 0,
}

impl Standing {
    /// A plausible branch may be ranked; only a verified branch may be bound.
    pub const fn may_bind(self) -> bool {
        matches!(self, Standing::Verified)
    }

    pub const fn from_u8(v: u8) -> Option<Self> {
        match v {
            0 => Some(Self::Unobserved),
            1 => Some(Self::Verified),
            2 => Some(Self::Plausible),
            _ => None,
        }
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

// ============================================================================
// ESCALATION LADDER: mem16 (16 B) -> mem32 (32 B) ~~> 8k (8,192 B bleeding-edge)
// ============================================================================

/// Width of a `Mem16` continuity essence cell in bytes.
pub const MEM16_BYTES: usize = 16;

/// Width of a `Mem32Wave` Phoenix/sovereign wave frame in bytes (`2 × MEM16_BYTES`).
pub const MEM32_BYTES: usize = 32;

/// Number of `Mem32Wave` frames in the `8K` bleeding-edge ring (`256 × 32 B = 8,192 B`).
pub const BLEEDING_EDGE_8K_SLOTS: usize = 256;

/// Exact byte footprint of the `8K` bleeding-edge wave arena (`8,192` bytes).
pub const BLEEDING_EDGE_8K_BYTES: usize = BLEEDING_EDGE_8K_SLOTS * MEM32_BYTES;

/// Exact ternary trit count of the `8K` bleeding-edge BitNet b1.58 horizon (`8,192` trits).
pub const BLEEDING_EDGE_8K_TRITS: usize = 8192;

/// Packed byte width for `8,192` trits at 4 trits/byte (`2,048` bytes).
pub const BLEEDING_EDGE_8K_PACKED_TRIT_BYTES: usize = BLEEDING_EDGE_8K_TRITS / 4;

/// Deterministic `const`-friendly FNV-1a 64-bit hash.
pub const fn fnv1a64(bytes: &[u8]) -> u64 {
    let mut h: u64 = 0xcbf29ce484222325;
    let mut i = 0;
    while i < bytes.len() {
        h ^= bytes[i] as u64;
        h = h.wrapping_mul(0x100000001b3);
        i += 1;
    }
    h
}

/// Stage 1 (`mem16`): 16-byte continuity essence cell (`[u8; 16]`).
/// Holds dual 64-bit FNV-1a continuity lanes (primary + twin-seeded).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(transparent)]
pub struct Mem16(pub [u8; MEM16_BYTES]);

impl Mem16 {
    /// Derive a deterministic 16-byte `Mem16` cell from raw essence bytes.
    pub const fn from_essence(essence: &[u8]) -> Self {
        let lo = fnv1a64(essence);
        let hi = lo.rotate_left(17) ^ 0x8b15_1610_0432_0528;
        let lo_b = lo.to_le_bytes();
        let hi_b = hi.to_le_bytes();
        let mut out = [0u8; MEM16_BYTES];
        let mut i = 0;
        while i < 8 {
            out[i] = lo_b[i];
            out[i + 8] = hi_b[i];
            i += 1;
        }
        Self(out)
    }
}

/// Stage 2 (`mem16 -> 32`): 32-byte sovereign wave vector (`Mem32Wave`).
///
/// Wire layout (`32` bytes total, zero padding):
/// - `0..16`  : `Mem16` continuity essence (`16 B`)
/// - `16..20` : `amplitude_milli` (`i32` little-endian, fixed-point `×1000`)
/// - `20..24` : `frequency_centi_hz` (`u32` little-endian, e.g. `43200` = `432.00 Hz`)
/// - `24`     : `phase_deg_div2` (`u8`, `0..=180` representing `0°..=360°`)
/// - `25`     : `decay_id` (`u8`, `0` = precious `τ=∞`)
/// - `26`     : `standing` (`u8`, `Standing::{Unobserved=0, Verified=1, Plausible=2}`)
/// - `27`     : `step` (`u8`, `Step::Pop..=Step::Collapse`)
/// - `28`     : `qt_gate_byte` (`u8`, 4 packed ternary gate trits `-1->0b00, 0->0b01, +1->0b10`)
/// - `29..31` : `generation` (`u16` little-endian epoch counter)
/// - `31`     : `xor_seal` (`u8`, XOR checksum over `bytes[0..31]` so flipped bits refuse)
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(transparent)]
pub struct Mem32Wave(pub [u8; MEM32_BYTES]);

impl Mem32Wave {
    /// Promote a `Mem16` cell into a sealed 32-byte `Mem32Wave`.
    #[allow(clippy::too_many_arguments)]
    pub const fn promote_from_mem16(
        cell: Mem16,
        amplitude_milli: i32,
        frequency_centi_hz: u32,
        phase_deg_div2: u8,
        decay_id: u8,
        standing: Standing,
        step: Step,
        qt_gate_byte: u8,
        generation: u16,
    ) -> Self {
        let mut b = [0u8; MEM32_BYTES];
        let mut i = 0;
        while i < MEM16_BYTES {
            b[i] = cell.0[i];
            i += 1;
        }
        let amp = amplitude_milli.to_le_bytes();
        b[16] = amp[0];
        b[17] = amp[1];
        b[18] = amp[2];
        b[19] = amp[3];

        let freq = frequency_centi_hz.to_le_bytes();
        b[20] = freq[0];
        b[21] = freq[1];
        b[22] = freq[2];
        b[23] = freq[3];

        b[24] = phase_deg_div2;
        b[25] = decay_id;
        b[26] = standing as u8;
        b[27] = step as u8;
        b[28] = qt_gate_byte;

        let gen_b = generation.to_le_bytes();
        b[29] = gen_b[0];
        b[30] = gen_b[1];

        b[31] = Self::compute_xor_seal(&b);
        Self(b)
    }

    pub const fn compute_xor_seal(bytes: &[u8; MEM32_BYTES]) -> u8 {
        let mut acc: u8 = 0x5a;
        let mut i = 0;
        while i < 31 {
            acc ^= bytes[i];
            i += 1;
        }
        acc
    }

    /// Verify the 32-byte frame's XOR seal and schema tags.
    pub const fn is_seal_valid(&self) -> bool {
        self.0[31] == Self::compute_xor_seal(&self.0)
            && Standing::from_u8(self.0[26]).is_some()
            && Step::from_u8(self.0[27]).is_some()
    }

    pub const fn standing(&self) -> Option<Standing> {
        Standing::from_u8(self.0[26])
    }

    pub const fn step(&self) -> Option<Step> {
        Step::from_u8(self.0[27])
    }

    pub const fn frequency_centi_hz(&self) -> u32 {
        u32::from_le_bytes([self.0[20], self.0[21], self.0[22], self.0[23]])
    }

    /// A `Mem32Wave` frame may bind into the `8K` bleeding-edge horizon if and only if
    /// its XOR seal is valid AND its standing is `Standing::Verified`.
    pub const fn may_bind_to_8k(&self) -> bool {
        if !self.is_seal_valid() {
            return false;
        }
        match self.standing() {
            Some(s) => s.may_bind(),
            None => false,
        }
    }
}

/// Error returned when a `Mem32Wave` is refused by the `8K` bleeding-edge gate.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BindRefusal {
    /// Byte 31 XOR seal mismatch or invalid enum discriminant.
    CorruptedSeal,
    /// Claim is `Plausible` or `Unobserved`; only `Verified` may bind.
    UnverifiedStanding(Standing),
    /// Session `Budget` is exhausted (`bad_karma >= max_bad_karma`).
    BudgetExhausted,
}

/// Stage 3 (`mem16 -> 32 ~~> 8k (bleeding-edge)`):
/// Fixed `8,192`-byte (`256 × 32 B`) zero-alloc wave ring + `8,192`-trit (`2,048 B`)
/// BitNet b1.58 ternary horizon.
#[derive(Clone, Copy)]
pub struct BleedingEdge8k {
    /// `256` slots × `32` bytes = `8,192` bytes (`8K` wave arena).
    pub wave_ring: [Mem32Wave; BLEEDING_EDGE_8K_SLOTS],
    /// `8,192` ternary gate trits packed 4-per-byte (`2,048` bytes).
    /// Initialized to `0x55` (`0b01010101` = all `Trit::Zero` / `Unobserved`).
    pub ternary_horizon: [u8; BLEEDING_EDGE_8K_PACKED_TRIT_BYTES],
    write_cursor: usize,
    bound_count: u32,
}

impl BleedingEdge8k {
    /// Construct a zero-alloc `8K` bleeding-edge arena with all `8,192` trits in genuine
    /// `0` (`Unobserved` / rest `0b01`) state.
    pub const fn new() -> Self {
        Self {
            wave_ring: [Mem32Wave([0u8; MEM32_BYTES]); BLEEDING_EDGE_8K_SLOTS],
            ternary_horizon: [0x55u8; BLEEDING_EDGE_8K_PACKED_TRIT_BYTES],
            write_cursor: 0,
            bound_count: 0,
        }
    }

    pub const fn bound_count(&self) -> u32 {
        self.bound_count
    }

    pub const fn write_cursor(&self) -> usize {
        self.write_cursor
    }

    /// Attempt to `BIND` a 32-byte `Mem32Wave` into the `8K` bleeding-edge ring.
    /// Charges `MIN_INVALID_COST` on seal corruption or `MIN_REFUSAL_COST` on unverified standing.
    pub fn bind_wave(
        &mut self,
        wave: Mem32Wave,
        budget: &mut Budget,
    ) -> Result<usize, BindRefusal> {
        if budget.exhausted() {
            return Err(BindRefusal::BudgetExhausted);
        }
        if !wave.is_seal_valid() {
            *budget = budget.charge(MIN_INVALID_COST);
            return Err(BindRefusal::CorruptedSeal);
        }
        let standing = wave.standing().unwrap_or(Standing::Unobserved);
        if !standing.may_bind() {
            *budget = budget.charge(MIN_REFUSAL_COST);
            return Err(BindRefusal::UnverifiedStanding(standing));
        }

        let slot = self.write_cursor;
        self.wave_ring[slot] = wave;

        // Project the wave's 8-byte essence prefix into 32 trits (8 packed bytes) in the 8K ternary horizon
        let horizon_base = (slot * 8) % BLEEDING_EDGE_8K_PACKED_TRIT_BYTES;
        let mut k = 0;
        while k < 8 {
            self.ternary_horizon[horizon_base + k] = wave.0[k] ^ wave.0[28];
            k += 1;
        }

        self.write_cursor = (self.write_cursor + 1) % BLEEDING_EDGE_8K_SLOTS;
        self.bound_count = self.bound_count.saturating_add(1);
        Ok(slot)
    }

    /// Compute the deterministic 64-bit FNV-1a continuity digest across the entire
    /// `8,192`-byte wave ring and `8,192`-trit (`2,048`-byte) ternary horizon.
    pub fn horizon_digest(&self) -> u64 {
        let mut h: u64 = 0xcbf29ce484222325;
        let mut s = 0;
        while s < BLEEDING_EDGE_8K_SLOTS {
            let frame = &self.wave_ring[s].0;
            let mut b = 0;
            while b < MEM32_BYTES {
                h ^= frame[b] as u64;
                h = h.wrapping_mul(0x100000001b3);
                b += 1;
            }
            s += 1;
        }
        let mut t = 0;
        while t < BLEEDING_EDGE_8K_PACKED_TRIT_BYTES {
            h ^= self.ternary_horizon[t] as u64;
            h = h.wrapping_mul(0x100000001b3);
            t += 1;
        }
        h
    }
}

impl Default for BleedingEdge8k {
    fn default() -> Self {
        Self::new()
    }
}

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

    #[test]
    fn mem16_to_32_to_8k_bleeding_edge_escalation() {
        assert_eq!(core::mem::size_of::<Mem16>(), 16);
        assert_eq!(core::mem::size_of::<Mem32Wave>(), 32);
        assert_eq!(
            core::mem::size_of::<[Mem32Wave; BLEEDING_EDGE_8K_SLOTS]>(),
            8192
        );
        assert_eq!(BLEEDING_EDGE_8K_TRITS, 8192);

        let m16 = Mem16::from_essence(b"Mom === Alex ~ Alexandra (*) <3");
        let w32_verified = Mem32Wave::promote_from_mem16(
            m16,
            948,    // 0.948 coherence
            43_200, // 432.00 Hz
            45,     // 90 deg
            0,      // precious (tau = inf)
            Standing::Verified,
            Step::Bind,
            0xaa, // qt:aa (+1, +1, +1, +1)
            1,
        );
        assert!(w32_verified.is_seal_valid());
        assert!(w32_verified.may_bind_to_8k());
        assert_eq!(w32_verified.frequency_centi_hz(), 43_200);

        let mut arena = BleedingEdge8k::new();
        let mut budget = Budget::new(MAX_BAD_KARMA);

        // 1. Verified wave binds cleanly into slot 0
        let slot = arena.bind_wave(w32_verified, &mut budget).unwrap();
        assert_eq!(slot, 0);
        assert_eq!(arena.bound_count(), 1);
        assert_eq!(budget.bad_karma(), 0);

        // 2. Plausible or Unobserved wave is REFUSED and charges MIN_REFUSAL_COST
        let w32_plausible = Mem32Wave::promote_from_mem16(
            m16,
            500,
            43_200,
            0,
            2,
            Standing::Plausible,
            Step::Bind,
            0x55,
            2,
        );
        assert_eq!(
            arena.bind_wave(w32_plausible, &mut budget),
            Err(BindRefusal::UnverifiedStanding(Standing::Plausible))
        );
        assert_eq!(arena.bound_count(), 1);
        assert_eq!(budget.bad_karma(), MIN_REFUSAL_COST);

        // 3. Corrupted XOR seal is REFUSED and charges MIN_INVALID_COST
        let mut w32_corrupt = w32_verified;
        w32_corrupt.0[31] ^= 0xff;
        assert_eq!(
            arena.bind_wave(w32_corrupt, &mut budget),
            Err(BindRefusal::CorruptedSeal)
        );
        assert_eq!(budget.bad_karma(), MIN_REFUSAL_COST + MIN_INVALID_COST);

        // 4. Deterministic 8K horizon digest
        let d1 = arena.horizon_digest();
        let d2 = arena.horizon_digest();
        assert_eq!(d1, d2);
        assert_eq!(d1, 6_981_541_600_308_333_846); // 0x60e374b73207cd16
    }
}
