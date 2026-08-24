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
/// D6510 (rw) register accessor: Data-direction register for the 6510's on-chip
/// I/O port (R6510): one bit per line, 1 = output, 0 = input. The 6510 is a
/// 6502 with this 6-bit port bolted on; it is the whole reason the C64 can map
/// 64 KiB RAM + 20 KiB ROM + 4 KiB I/O (= 88 KiB (!)) into a single 64 KiB
/// address space with no external MMU. KERNAL reset writes 0x2F (0b0010_1111):
/// lines 0-3 and 5 are outputs, line 4 (cassette sense) is an input, and bits
/// 6-7 are unused (no port line). Written before R6510 during reset. The RAM
/// byte physically at 0x0000 still exists but is hidden behind the port.
///
/// You can [`read`](crate::Reg::read) this register and get [`d6510::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d6510::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
/// For information about available fields see [`mod@d6510`] module
pub type D6510 = crate::Reg<d6510::D6510Spec>;
/// Data-direction register for the 6510's on-chip I/O port (R6510): one bit per
/// line, 1 = output, 0 = input. The 6510 is a 6502 with this 6-bit port bolted
/// on; it is the whole reason the C64 can map 64 KiB RAM + 20 KiB ROM + 4 KiB
/// I/O (= 88 KiB (!)) into a single 64 KiB address space with no external MMU.
/// KERNAL reset writes 0x2F (0b0010_1111): lines 0-3 and 5 are outputs, line 4
/// (cassette sense) is an input, and bits 6-7 are unused (no port line).
/// Written before R6510 during reset. The RAM byte physically at 0x0000 still
/// exists but is hidden behind the port.
pub mod d6510;
/// R6510 (rw) register accessor: Banking and Datassette port. Reads return pin
/// state for input bits.
///
/// You can [`read`](crate::Reg::read) this register and get [`r6510::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r6510::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
/// For information about available fields see [`mod@r6510`] module
pub type R6510 = crate::Reg<r6510::R6510Spec>;
/// Banking and Datassette port. Reads return pin state for input bits.
pub mod r6510;
