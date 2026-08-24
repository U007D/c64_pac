#!/usr/bin/env bash
# c64_pac/svd/generate_c64_pac.sh — sole writer of c64_pac/src/ (the generated PAC).
# Regenerates the peripheral-access API from c64.svd. It only emits source, so it
# does NOT need the C64 cross-toolchain (`nix develop`): it installs its own
# generator tools (below), and needs only cargo and a nightly rustfmt on PATH.
set -euo pipefail

# Crate root is the parent of this svd/ directory, wherever the script lives.
crate="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
svd="$crate/svd/c64.svd"

# Generator tools. Install only what is missing: a tool already on PATH is the
# user's, however they put it there (cargo, brew, nix), and reinstalling it would
# either shadow it with a duplicate in ~/.cargo/bin or fail outright on a machine
# that is offline. Unpinned, so a machine that lacks one gets the current release.
#
# A failed install is not fatal. The preflight below decides whether we can
# actually proceed, and it can name what is missing far more clearly than a
# half-finished cargo build can.
#
# `fd` ships as the crate `fd-find`; the other three are named for their binaries.
tools="svd2rust:svd2rust form:form sd:sd fd:fd-find"

for entry in $tools; do
    tool="${entry%%:*}"
    pkg="${entry##*:}"
    if ! command -v "$tool" >/dev/null 2>&1; then
        echo "installing $pkg (provides $tool)"
        cargo install "$pkg" --locked || true
    fi
done

missing=""
for entry in $tools; do
    tool="${entry%%:*}"
    command -v "$tool" >/dev/null 2>&1 || missing="$missing ${entry##*:}"
done
if [ -n "$missing" ]; then
    echo "error: generator tool(s) unavailable. Install with:" >&2
    echo "           cargo install$missing --locked" >&2
    exit 1
fi

# Warn on generator drift. The emitted API shape tracks svd2rust's version and
# nothing in the output announces a change of shape — you just get a 20k-line
# diff. svd2rust stamps its version into the docs.rs link in src/lib.rs, so the
# version that produced the current tree is recoverable; compare and say so up
# front. This is the check that matters, and it catches more than pinning would:
# a leftover svd2rust 0.35.0 predates the `Periph<RB, const A>` accessor src/ is
# built on and would revert every peripheral to the pre-0.36 shape, while a
# *newer* release can restructure src/ just as thoroughly.
if [ -f "$crate/src/lib.rs" ]; then
    was="$(grep -om1 'svd2rust/[0-9][0-9.]*' "$crate/src/lib.rs" | cut -d/ -f2 || true)"
    now="$(svd2rust --version | awk '{ print $2 }')"
    if [ -n "$was" ] && [ "$was" != "$now" ]; then
        echo "warning: src/ was generated with svd2rust $was; this run uses $now." >&2
        echo "         The generated API shape tracks the version — expect a large diff," >&2
        echo "         and re-check the fix-ups below, which key on generated text." >&2
    fi
fi

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

# Fix 1: inject the hand-written Bcd newtype module (BCD values for the CIA TOD
# registers). It doesn't depend on the SVD, so it lives as a static heredoc here
# and is rewritten on every regen alongside the generated tree. TOD fields keep
# their raw BCD `.bits()`; callers wrap opt-in with `bcd::Seconds`, etc.
#
# Injected here, before the move and the format below, rather than down with the
# other fix-ups: it matches no generated text, so nothing about it needs rustfmt
# to have run first — and going in early means rustfmt formats bcd.rs too,
# instead of leaving one hand-written file in the tree unformatted. Both halves
# have to move together: rustfmt reaches bcd.rs only by following the
# `pub mod bcd;` declaration from the crate root.
cat > src/bcd.rs <<'RUST'
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
printf '\npub mod bcd;\n' >> src/lib.rs

# Move into the crate, then format the whole tree — generated and hand-written
# alike — with the crate's own rustfmt.toml + edition.
# Formatting BEFORE the spacing-sensitive fix-ups below is what lets their tight
# patterns match: svd2rust + form emit `# [no_mangle]` / `pub mod extcol ;`, which
# rustfmt collapses to `#[no_mangle]` / `pub mod extcol;`.
rm -rf "$crate/src"
mv src "$crate/src"
( cd "$crate" && cargo +nightly fmt )

# Fix 2: edition 2024 requires the unsafe-wrapped attribute spelling. fd runs sd
# once per file, independent of the shell's word-splitting and of ARG_MAX.
fd --no-ignore --hidden -e rs . "$crate/src" -x sd '#\[no_mangle\]' '#[unsafe(no_mangle)]' {}

# Fix 3: svd2rust globally reserves `set` as a field-writer method name (it
# collides with FieldWriter::set, the raw multi-bit value setter). That reserves
# it even for single-bit BitWriter fields, which have no inherent `set` — so the
# ICR mask-select variant `Set` is emitted as `set_()`. BitWriter has no `set`,
# so rename it back to the clean `set()` the API wants. `fn set_(` matches only
# this method (set_bit stays `fn set_bit(`).
fd --no-ignore --hidden -e rs . "$crate/src" -x sd 'pub fn set_\(self\)' 'pub fn set(self)' {}

# Fix 4: re-export the shared Color enum at the vic module level, so consumers
# write `vic::Color` instead of `vic::extcol::Color`. svd2rust must define the enum
# on some field (EXTCOL's); the border has no special claim on the palette, so
# hoist it. Every colour register (border, background, sprites) and colour RAM
# derive this one enum, so the single re-export names them all.
sd 'pub mod extcol;' $'pub mod extcol;\npub use extcol::Color;' "$crate/src/vic.rs"

# Fix 5: hand-write the CPUPORT peripheral. Its baseAddress is 0, so the
# generated `Periph<cpuport::RegisterBlock, 0>` would reach both registers
# through a `&RegisterBlock` at address 0 — a null reference. CPUPORT gets a
# concrete handle instead: R6510 at 0x0001 keeps the ordinary `&Reg` API, and
# D6510 at 0x0000 becomes a `D6510Port` ZST doing volatile access, which is
# defined at any address (Rust 1.90, rust-lang/rust#141260). Touches three
# files, each rebuilt from the heredocs below so a regeneration cannot silently
# drop the fix.
#
# The injected text is already rustfmt-formatted to this crate's rustfmt.toml, so
# it needs no second `cargo fmt` pass.

# 5a: `raw::{R, W}::_reg` is `pub(super)`, so D6510Port cannot build a reader or
# a writer on its own. Add crate-visible constructors alongside the types.
cat >> "$crate/src/generic.rs" <<'RUST'

/// Constructors for the register reader/writer, from raw bits.
///
/// `Reg`'s own methods build `R`/`W` inline, so svd2rust emits no constructor.
/// The hand-written `cpuport::D6510Port` reaches its register through
/// `ptr::read_volatile`/`write_volatile` rather than through a `&Reg` — see
/// that type for why — so it has to build them itself, and `raw::{R, W}::_reg`
/// is `pub(super)`, visible only from here in `generic`.
impl<REG: RegisterSpec> R<REG> {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn from_bits(bits: REG::Ux) -> Self {
        Self { bits, _reg: marker::PhantomData }
    }
}
impl<REG: RegisterSpec> W<REG> {
    #[allow(unused)]
    #[inline(always)]
    pub(crate) const fn from_bits(bits: REG::Ux) -> Self {
        Self { bits, _reg: marker::PhantomData }
    }
}
RUST

# 5b: swap cpuport.rs's RegisterBlock — and its two `&Reg` accessors, now
# unreachable — for D6510Port, keeping every generated register and field module
# from the `D6510 (rw) register accessor` doc block onward.
{
    cat <<'RUST'
use core::{marker::PhantomData,
           ptr::{read_volatile, without_provenance_mut, write_volatile}};

/// Address of D6510, the 6510 data-direction register.
///
/// `without_provenance_mut` rather than an `as` cast: the address names memory
/// outside every Rust allocation, so there is no provenance to carry, and
/// saying so explicitly is what makes the pointer well-formed under strict
/// provenance.
const D6510_ADDR: *mut u8 = without_provenance_mut(0x0000);

/// Handle to D6510, the 6510's data-direction register, at address 0x0000.
///
/// A zero-sized token rather than a `&D6510`: `read`/`write`/`modify` reach the
/// register with `ptr::read_volatile`/`write_volatile` instead of through a
/// reference. "Here, any address value is possible, including 0 and
/// `usize::MAX`, so long as the semantics of such a pointer are well-defined by
/// the target hardware" ([`ptr::write_volatile` § Safety], Rust 1.90,
/// [rust-lang/rust#141260]). The C64 defines address 0 as D6510.
///
/// R6510 at 0x0001 keeps the ordinary `&Reg` API.
///
/// # Caveat on pre-1.90 toolchains
///
/// #141260 changed no codegen, but older `core` — including the rust-mos
/// 1.87.0-dev fork — still asserts that `write_volatile`'s pointer is non-null
/// whenever debug assertions are on, and aborts. Build `--release` (or set
/// `debug-assertions = false`) until the toolchain is past 1.90.
///
/// [`ptr::write_volatile` § Safety]: https://doc.rust-lang.org/std/ptr/fn.write_volatile.html#safety
/// [rust-lang/rust#141260]: https://github.com/rust-lang/rust/pull/141260
pub struct D6510Port {
    /// `*const ()` rather than `()`: makes the token `!Send + !Sync`, matching
    /// the `Periph` handles the other peripherals hand out.
    _marker: PhantomData<*const ()>,
}

impl D6510Port {
    #[inline(always)]
    pub(crate) const fn new() -> Self { Self { _marker: PhantomData } }

    /// The underlying address of the register, for parity with `Reg::as_ptr`.
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut u8 { D6510_ADDR }

    /// Reads the contents of the register.
    #[inline(always)]
    pub fn read(&self) -> d6510::R {
        // SAFETY: 0x0000 is the D6510 hardware register, outside every Rust
        // allocation, and the 6510 defines a byte read of it. Volatile, so the
        // read is neither elided nor reordered against other volatile accesses.
        crate::R::from_bits(unsafe { read_volatile(D6510_ADDR) })
    }

    /// Writes bits to the register. Fields the closure does not name are set to
    /// their reset value.
    #[inline(always)]
    pub fn write<F>(&self, f: F) -> u8
        where F: FnOnce(&mut d6510::W) -> &mut d6510::W, {
        let mut w = crate::W::from_bits(<d6510::D6510Spec as crate::Resettable>::RESET_VALUE
                                        & !<d6510::D6510Spec as crate::Writable>::ONE_TO_MODIFY_FIELDS_BITMAP
                                        | <d6510::D6510Spec as crate::Writable>::ZERO_TO_MODIFY_FIELDS_BITMAP);
        let value = f(&mut w).bits;
        // SAFETY: as `read`; the 6510 defines a byte write to 0x0000.
        unsafe { write_volatile(D6510_ADDR, value) };
        value
    }

    /// Reads the register, modifies it, writes it back. Fields the closure does
    /// not name keep the value they had.
    #[inline(always)]
    pub fn modify<F>(&self, f: F) -> u8
        where for<'w> F: FnOnce(&d6510::R, &'w mut d6510::W) -> &'w mut d6510::W, {
        // SAFETY: as `read`.
        let bits = unsafe { read_volatile(D6510_ADDR) };
        let r = crate::R::from_bits(bits);
        let mut w = crate::W::from_bits(bits
                                        & !<d6510::D6510Spec as crate::Writable>::ONE_TO_MODIFY_FIELDS_BITMAP
                                        | <d6510::D6510Spec as crate::Writable>::ZERO_TO_MODIFY_FIELDS_BITMAP);
        let value = f(&r, &mut w).bits;
        // SAFETY: as `write`.
        unsafe { write_volatile(D6510_ADDR, value) };
        value
    }
}

impl core::fmt::Debug for D6510Port {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
RUST
    sed -n '/^\/\/\/ D6510 (rw) register accessor:/,$p' "$crate/src/cpuport.rs"
} > "$work/cpuport.rs"
mv "$work/cpuport.rs" "$crate/src/cpuport.rs"

# 5c: swap the `pub type Cpuport = Periph<…, 0>` alias and its Debug impl in
# lib.rs for the concrete struct. The alias sits between `pub mod generic;` and
# the doc block for `pub mod cpuport;`; everything outside that span is generated
# output and is kept verbatim.
cat > "$work/cpuport_lib.rs" <<'RUST'
/// MOS 6510 on-chip I/O port at addresses 0x0000-0x0001: memory banking control
/// and Datassette lines. KERNAL reset writes D6510=0x2F, R6510=0x37.
///
/// Hand-written, not generated: with `baseAddress` 0 the generated
/// `Periph<RegisterBlock, 0>` would reach both registers through a
/// `&RegisterBlock` at address 0 — a null reference. This type hands them out
/// directly instead: `r6510()` as an ordinary `&Reg` at 0x0001, `d6510()` as a
/// [`cpuport::D6510Port`] doing volatile access at 0x0000.
///
/// Re-applied after every regeneration by `svd/generate_c64_pac.sh`, Fix 5.
pub struct Cpuport {
    /// `*const ()` rather than `()`: makes the handle `!Send + !Sync`, matching
    /// `Periph`.
    _marker: core::marker::PhantomData<*const ()>,
}
unsafe impl Send for Cpuport {}
impl Cpuport {
    /// Steal an instance of this peripheral
    ///
    /// # Safety
    ///
    /// Ensure that the new instance of the peripheral cannot be used in a way
    /// that may race with any existing instances, for example by only
    /// accessing read-only or write-only registers, or by consuming the
    /// original peripheral and using critical sections to coordinate
    /// access between multiple new instances.
    ///
    /// Additionally, other software such as HALs may rely on only one
    /// peripheral instance existing to ensure memory safety; ensure
    /// no stolen instances are passed to such software.
    pub unsafe fn steal() -> Self { Self { _marker: core::marker::PhantomData } }

    /// 0x00 - Data-direction register for the 6510's on-chip I/O port (R6510):
    /// one bit per line, 1 = output, 0 = input. The 6510 is a 6502 with this
    /// 6-bit port bolted on; it is the whole reason the C64 can map 64 KiB RAM
    /// + 20 KiB ROM + 4 KiB I/O (= 88 KiB (!)) into a single 64 KiB address
    /// space with no external MMU. KERNAL reset writes 0x2F (0b0010_1111):
    /// lines 0-3 and 5 are outputs, line 4 (cassette sense) is an input, and
    /// bits 6-7 are unused (no port line). Written before R6510 during reset.
    /// The RAM byte physically at 0x0000 still exists but is hidden behind the
    /// port.
    ///
    /// Returns a [`cpuport::D6510Port`] token rather than a `&D6510`; it
    /// carries the same `read`/`write`/`modify` API.
    #[inline(always)]
    pub const fn d6510(&self) -> cpuport::D6510Port { cpuport::D6510Port::new() }

    /// 0x01 - Banking and Datassette port. Reads return pin state for input
    /// bits.
    #[inline(always)]
    pub fn r6510(&self) -> &cpuport::R6510 {
        // SAFETY: 0x0001 is the R6510 hardware register — non-null,
        // byte-aligned, outside every Rust allocation, and `Reg` wraps a
        // `VolatileCell` so shared access is the correct shape. The same
        // reference `Periph::deref` forms for every other peripheral.
        unsafe { &*(0x0001 as *const cpuport::R6510) }
    }
}
impl core::fmt::Debug for Cpuport {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cpuport").finish()
    }
}
RUST
awk -v bodyfile="$work/cpuport_lib.rs" '
    state == 0 {
        print
        if ($0 == "pub mod generic;") {
            while ((getline line < bodyfile) > 0) print line
            state = 1
        }
        next
    }
    state == 1 { if (/f\.debug_struct\("Cpuport"\)\.finish\(\)/) state = 2; next }
    state == 2 { if ($0 == "}") state = 3; next }
    state == 3 { print }
' "$crate/src/lib.rs" > "$work/lib.rs"
mv "$work/lib.rs" "$crate/src/lib.rs"

# Fix 6: annotate shared field types. svd2rust re-exports one canonical field
# writer/reader under many alias names; jump-to-definition on an alias lands on
# the canonical type, whose doc describes only its first use. For any type shared
# across more than one module, append an intra-doc "Shared field type" directory
# linking every alias, so the reader can click through to each one's own
# description instead of being stranded on an unrelated register's prose. Runs as
# a single awk pass over the whole tree (it needs cross-file state).
awk -f "$crate/svd/patch_shared_docs.awk" $(fd --no-ignore --hidden -e rs . "$crate/src")
