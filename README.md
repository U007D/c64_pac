# c64_pac

## AI Policy: RDT
AI was used in this crate for Research, Documentation and Tooling only.  This project contains no
Code artifacts (code which end users run).  The `.svd` file in this crate documenting the registers
and fields of the Commodore 64 was generated with (the careful use of) AI.

Note the Rust code emitted by `svd2rust` (the Code artifact of this crate) is not AI generated.

## Description
A **peripheral access crate (PAC)** for the Commodore 64 — typed, zero runtime
overhead (ZRO) access to the C64's memory-mapped I/O, generated from a
[CMSIS-SVD](https://open-cmsis-pack.github.io/svd-spec/)
description file with [svd2rust](https://github.com/rust-embedded/svd2rust). `#![no_std]`, no
`alloc`; register reads/writes compile down to the same loads and stores you'd write by
hand, but behind names and types instead of magic addresses and bit masks.

The SVD describes **8 peripherals** — CPU port, VIC-II, SID, CIA1, CIA2, color RAM, and the
two I/O expansion windows (IO1/IO2) — across **79 registers** and **151 fields**, with one
shared 16-value `Color` enum and the full 1024-nybble color RAM.

## Quick Start - Generate PAC API
From the crate root, type `svd/generate_c64_pac.sh`.  The script installs any generator tool
you are missing and leaves alone any you already have, so it needs only `cargo` and a
nightly `rustfmt` on PATH.
If the `src` file exists, it will automatically be removed and replaced in its entirety with the
new generated output.  No interleaving of old and new `src` folder contents will occur.

```rust
// One shared handle to all peripherals. `steal()` is always available;
#[cfg(not(feature="critical-section"))]
let p = unsafe { c64_pac::Peripherals::steal() };

// `take()` needs the `critical-section` feature + a critical-section impl.
#[cfg(feature="critical-section")]
let p = c64_pac::Peripherals::take();

// Border colour: no address, no magic number — a name and a typed value.
p.vic.extcol().write(|w| w.color().light_blue());

// Joystick 2 fire (CIA1 port B, active-low) — the polarity is hidden in the type.
if p.cia1.ciaprb().read().fire().is_pressed() {
    p.vic.extcol().write(|w| w.color().red());
}
```

## Using it

```toml
[dependencies]
c64_pac = { path = "../c64_pac" }              # or a version once published

# Optional: enable the safe singleton `Peripherals::take()`.
# c64_pac = { path = "../c64_pac", features = ["critical-section"] }
```

- **`Peripherals::steal()`** — always available, `unsafe`; you promise there's no aliasing.
- **`Peripherals::take()`** — safe, returns `Some` once then `None`. Gated behind the
  **`critical-section`** feature, which pulls in the [`critical-section`](https://docs.rs/critical-section)
  crate. When you enable it, your binary must also provide a `critical-section` *implementation*
  (on the single-core 6510 that's a tiny SEI/CLI shim). With the feature off — the default —
  the crate has no such dependency and only `steal()` exists.

Register access follows the standard svd2rust shape: `read()` returns a reader with a predicate
per field (`.fire().is_pressed()`), `write(|w| …)` takes a closure of typed setters
(`.color().red()`), and read-write registers also offer `modify(|r, w| …)`.

## The five footgun registers

Some C64 registers don't behave like plain storage — a read or a write has a side effect, or
the read value has nothing to do with the last write. Where read and write are genuinely
*different registers* at one address, the PAC models them as an **alternate read-only + write-only
pair** (suffixed `_R` / `_W`) so `.modify()` — which would read-then-write — is simply not
offered. The rest carry a `SIDE-EFFECT:` note in their generated docs. Watch these:

| Register | Address | The gotcha |
|---|---|---|
| VIC `RASTER_R` / `RASTER_W` | `$D012` | **Read** = current raster line. **Write** = the line that raises a raster IRQ. Different meanings → split RO/WO. |
| VIC `VICIRQ_R` / `VICIRQ_W` | `$D019` | **Read** = pending IRQ latches. **Write** = *acknowledge* (write-1-to-clear). Split RO/WO. |
| VIC `SPSPCL` / `SPBGCL` | `$D01E` / `$D01F` | Sprite–sprite / sprite–background collision latches, **cleared by reading**. Read once and cache; a second read returns zero. |
| CIA `CIAICR_R`/`_W`, `CI2ICR_R`/`_W` | `$DC0D` / `$DD0D` | **Read** = pending interrupts *and clears them*. **Write** = set/clear the mask (bit 7 selects set-vs-clear for the bits you wrote). Split RO/WO. |
| CIA `TOD` (`TODTEN…TODHRS`) | `$DC08`–`$DC0B` | **Reading** the hours register *latches* the time until you read tenths; **writing** targets the clock or the alarm depending on CRB bit 7. |

Each of these is also documented at its `<description>` in `svd/c64.svd` and in the generated
per-register docs.

## Where the names come from

Register and field mnemonics follow *Mapping the Commodore 64* (Sheldon Leemon, COMPUTE!
Publications) as digitized in Michael Steil's [c64ref](https://github.com/mist64/c64ref)
([browsable](https://www.pagetable.com/c64ref/c64io/)), normalized where needed to valid Rust
identifiers. They are the datasheet/PRG-style hardware mnemonics (`EXTCOL`, `MSIGX`, `SCROLY`,
`SIGVOL`…), not the KERNAL/POKE labels. The SVD's own `<description>` records the modelling
assumptions (PLA banking, register mirroring, interrupt vectors, PAL/NTSC timing, chip
revisions) that a register map can't express.

## Regenerating

`svd/c64.svd` is the source of truth; the code in `src/` is generated from it and should never
be hand-edited. To regenerate after changing the SVD:

```sh
svd/generate_c64_pac.sh     # needs cargo and a nightly rustfmt on PATH
```

The script first `cargo install`s, unpinned, whichever of its generator tools are absent —
svd2rust, [form](https://github.com/djmcgill/form), sd, fd — and leaves any you already have
in place, however you installed them. If your svd2rust differs from the version that
produced the current `src/` it says so and continues: the generated API shape tracks the
generator's version, so that is the difference between a small diff and a total rewrite. It
then runs svd2rust + form and applies a
few fix-ups (crate-level `#![no_std]`, the hand-written `bcd` module, edition-2024
`unsafe(no_mangle)`, hoisting the shared `Color` enum to `vic::Color`, and hand-writing the
CPUPORT peripheral, whose base address of 0 rules out the generated `Deref`), then rewrites
`src/` in place. It does **not** require the C64 cross-toolchain — it only emits source.

## License

Dual origin, stated explicitly because the two halves have different licenses:

- **Generated code (`src/`)** — licensed under either of [MIT](LICENSE-MIT) or
  [Apache-2.0](LICENSE-APACHE) at your option, the standard Rust-ecosystem dual license.
- **Register data (`svd/c64.svd`)** — dedicated to the public domain under
  [CC0-1.0](https://creativecommons.org/publicdomain/zero/1.0/). Hardware register facts aren't
  anyone's copyright.

Unless you explicitly state otherwise, any contribution you submit for inclusion shall be dual
licensed as above, without any additional terms or conditions.
