#!/usr/bin/env bash
# c64_pac/svd/generate_c64_pac.sh — sole writer of c64_pac/src/ (the generated PAC).
# Regenerates the peripheral-access API from c64.svd. It only emits source, so it
# does NOT need the C64 cross-toolchain (`nix develop`): just svd2rust, form, sd,
# fd, and a nightly rustfmt on PATH.
set -euo pipefail

# Crate root is the parent of this svd/ directory, wherever the script lives.
crate="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
svd="$crate/svd/c64.svd"

work="$(mktemp -d)"
trap 'rm -rf "$work"' EXIT
cd "$work"

# svd2rust emits lib.rs; form splits it into a module tree whose root is, again,
# lib.rs (form hardcodes that name) plus one file/dir per peripheral. As its own
# crate root the PAC uses `crate::` correctly, so no path requalification is
# needed (that was only for embedding it as an in-crate `mod`).
#
# --impl-debug emits Debug for the register readers and RegisterBlocks.
svd2rust --target none --impl-debug -i "$svd"
form -i lib.rs -o src/

# Crate-level attributes svd2rust doesn't emit (it targets a `mod`-style include):
# a standalone crate root needs `#![no_std]`, and `#![allow(warnings)]` keeps the
# generated code quiet. Prepend both now; they don't depend on formatting.
sd '\A' $'#![no_std]\n#![allow(warnings)]\n' src/lib.rs

# Move into the crate, then format with the crate's own rustfmt.toml + edition.
# Formatting BEFORE the spacing-sensitive fix-ups below is what lets their tight
# patterns match: svd2rust + form emit `# [no_mangle]` / `pub mod extcol ;`, which
# rustfmt collapses to `#[no_mangle]` / `pub mod extcol;`.
rm -rf "$crate/src"
mv src "$crate/src"
( cd "$crate" && cargo +nightly fmt )

# Fix 1: edition 2024 requires the unsafe-wrapped attribute spelling. fd runs sd
# once per file, independent of the shell's word-splitting and of ARG_MAX.
fd --no-ignore --hidden -e rs . "$crate/src" -x sd '#\[no_mangle\]' '#[unsafe(no_mangle)]' {}

# Fix 1b: svd2rust globally reserves `set` as a field-writer method name (it
# collides with FieldWriter::set, the raw multi-bit value setter). That reserves
# it even for single-bit BitWriter fields, which have no inherent `set` — so the
# ICR mask-select variant `Set` is emitted as `set_()`. BitWriter has no `set`,
# so rename it back to the clean `set()` the API wants. `fn set_(` matches only
# this method (set_bit stays `fn set_bit(`).
fd --no-ignore --hidden -e rs . "$crate/src" -x sd 'pub fn set_\(self\)' 'pub fn set(self)' {}

# Fix 2: re-export the shared Color enum at the vic module level, so consumers
# write `vic::Color` instead of `vic::extcol::Color`. svd2rust must define the enum
# on some field (EXTCOL's); the border has no special claim on the palette, so
# hoist it. Every colour register (border, background, sprites) and colour RAM
# derive this one enum, so the single re-export names them all.
sd 'pub mod extcol;' $'pub mod extcol;\npub use extcol::Color;' "$crate/src/vic.rs"

# Fix 3: inject the hand-written Bcd newtype module (BCD values for the CIA TOD
# registers). It doesn't depend on the SVD, so it lives as a static heredoc here
# and is rewritten on every regen alongside the generated tree. TOD fields keep
# their raw BCD `.bits()`; callers wrap opt-in with `bcd::Seconds`, etc.
cat > "$crate/src/bcd.rs" <<'RUST'
//! Binary-coded-decimal values for the CIA TOD registers.
//!
//! Construct and read as ordinary decimal; the BCD packing stays internal.

/// A decimal value constrained to `MIN..=MAX`, stored as its packed BCD byte.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Bcd<const MIN: u8, const MAX: u8>(u8);

impl<const MIN: u8, const MAX: u8> Bcd<MIN, MAX> {
    /// From a decimal value; `None` if outside `MIN..=MAX`. `const`, so
    /// `Seconds::new(59).unwrap()` is range-checked at compile time.
    pub const fn new(decimal: u8) -> Option<Self> {
        if decimal < MIN || decimal > MAX {
            return None;
        }
        Some(Self((decimal / 10) << 4 | (decimal % 10)))
    }

    /// The decimal value.
    pub const fn get(self) -> u8 {
        (self.0 >> 4) * 10 + (self.0 & 0x0F)
    }

    /// The raw BCD byte, to write into the register.
    pub const fn to_bcd(self) -> u8 {
        self.0
    }

    /// From a BCD byte read from the register; `None` if a nibble is not a
    /// decimal digit or the decoded value is out of range.
    pub const fn from_bcd(byte: u8) -> Option<Self> {
        if (byte >> 4) > 9 || (byte & 0x0F) > 9 {
            return None;
        }
        Self::new((byte >> 4) * 10 + (byte & 0x0F))
    }
}

impl<const MIN: u8, const MAX: u8> core::fmt::Debug for Bcd<MIN, MAX> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.get())
    }
}

/// TOD tenths of a second (0-9).
pub type Tenths = Bcd<0, 9>;
/// TOD seconds (0-59).
pub type Seconds = Bcd<0, 59>;
/// TOD minutes (0-59).
pub type Minutes = Bcd<0, 59>;
/// TOD hours, 12-hour (1-12).
pub type Hours = Bcd<1, 12>;
RUST

# Expose the module at the crate root (appended after the generated items).
printf '\npub mod bcd;\n' >> "$crate/src/lib.rs"

# Fix 4: annotate shared field types. svd2rust re-exports one canonical field
# writer/reader under many alias names; jump-to-definition on an alias lands on
# the canonical type, whose doc describes only its first use. For any type shared
# across more than one module, append an intra-doc "Shared field type" directory
# linking every alias, so the reader can click through to each one's own
# description instead of being stranded on an unrelated register's prose. Runs as
# a single awk pass over the whole tree (it needs cross-file state).
awk -f "$crate/svd/patch_shared_docs.awk" $(fd --no-ignore --hidden -e rs . "$crate/src")
