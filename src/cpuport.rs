#[repr(C)]
#[derive(Debug)]
/// Register block
pub struct RegisterBlock {
    d6510: D6510,
    r6510: R6510,
}
impl RegisterBlock {
    /// 0x00 - Data-direction register for the 6510's on-chip I/O port (R6510):
    /// one bit per line, 1 = output, 0 = input. The 6510 is a 6502 with this
    /// 6-bit port bolted on; it is the whole reason the C64 can map 64 KiB RAM
    /// + 20 KiB ROM + 4 KiB I/O (= 88 KiB (!)) into a single 64 KiB address
    /// space with no external MMU. KERNAL reset writes 0x2F (0b0010_1111):
    /// lines 0-3 and 5 are outputs, line 4 (cassette sense) is an input, and
    /// bits 6-7 are unused (no port line). Written before R6510 during reset.
    /// The RAM byte physically at 0x0000 still exists but is hidden behind the
    /// port.
    #[inline(always)]
    pub const fn d6510(&self) -> &D6510 { &self.d6510 }

    /// 0x01 - Banking and Datassette port. Reads return pin state for input
    /// bits.
    #[inline(always)]
    pub const fn r6510(&self) -> &R6510 { &self.r6510 }
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
