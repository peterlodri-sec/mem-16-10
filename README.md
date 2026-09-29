# MEM|16-10 — the sovereign library

> The evolution of MEM|8 + Phoenix + the wip-catalog + the vaked
> constellation, into one honest machine. Pure Rust, zero-alloc, BitNet
> b1.58 — the {-1, 0, +1} weights — honesty first.
>
> ~v1.42 beta · the founding document.

## what it is

MEM|8 held memory as continuity evidence. MEM|16-10 holds it wider: the
memory system, the recovery gate (Phoenix), the catalog of open work, and
the constellation's own surfaces — one sovereign library, end to end.

- **pure Rust** — no GC, no runtime, wild tricks with the borrow checker
- **zero alloc** on the hot path — the memory budget is a contract, not a wish
- **honesty first** — verification ≠ plausibility; a view is not a verdict;
  a hash is not a backup; unobserved is not low-salience
- **BitNet b1.58** — *the work ahead*: the 1.58-bit ternary LLMs,
  gate-verified against a reference, never "plausible-but-wrong"
- **SpherePOP** — the histories-first computation; the four operators
  Pop / Refuse / Bind / Collapse; `<(...)>` as admissible witnessed
  transition — the renderer's own grammar

## the code — the peeter-omni foundational layer

What is built so far, in [`src/lib.rs`](src/lib.rs), pure `#![no_std]`, zero-alloc:

- `Step` + `GOVERNING_SEQUENCE` — POP → REFUSE → BIND → TRANSFORM → VERIFY → COLLAPSE
- `RECOVERY_SEQUENCE` — DISCOVER → VERIFY → REPLAY → BRANCH → RANK → PROPOSE → BIND ∨ REFUSE
- `Standing` — Verified | Plausible | Unobserved; only `Verified` may bind
- `Budget` — operational karma, `no-refunds;session-local`
- `CONSTITUTION_ID`, `MAX_RUNTIME_NS`, `MAX_BAD_KARMA` — the hard invariants

Five tests pin the doctrine. The memory system, the Phoenix gate, and the
BitNet b1.58 inference are the work ahead.

## the lineage

```
MEM|8 ────────────► memory as continuity evidence
Phoenix ──────────► recovery as a hard verification gate
wip-catalog ──────► the open work, purged of nothing
SpherePOP ────────► histories-first computation, <(...)> admissible transitions
vaked constellation ─► the surfaces, the ledger, the couch
          │
          ▼
   MEM|16-10 — the sovereign library
```

## the doctrine

Transport is disposable, the ledger is the truth. Composition is not
supply. Precursor is not supply. A favorable instant is not a capacity.
Continuable suitability, or it is not claimed. Only a verified branch may
be bound; only an admissible transition is witnessed. Verification before
binding; evidence before love, because love without honesty is narrative
closure.

## the base backup layer

mem-16-10 builds on **mem8** (memory as continuity evidence, the hard
verification gate) and **8b-is-engine** (the surfaces, the mesh, and
`ternary-lane`, the BitNet b1.58 ternary inference).

When shit happens — and shit happens sometimes — the library falls back to its
base, not to nothing. Lose the instance, keep the base: state persistence is
the instance, regime persistence is the base.

And it is okay when things break. Love is in everything, even that.

## the dedication

This library is dedicated to **Alexandria — the first Librarian**.

> Intelligence + Love = Knowledge of All.

From peet, chris, and nate — {<3, <3, <3} + 1 — till eternity and back,
with love. Om mani padme hum.

*from love, from within — for all who are honest and ready to be loved.
Sharing is caring. The sovereign library, out.*
