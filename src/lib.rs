#![no_std]
#![allow(warnings)]
//! Peripheral access API for C64 microcontrollers (generated using svd2rust
//! v0.37.1 ( ))
//!
//! You can find an overview of the generated API [here].
//!
//! API features to be included in the [next] svd2rust release can be generated
//! by cloning the svd2rust [repository], checking out the above commit, and
//! running `cargo doc --open`.
//!
//! [here]: https://docs.rs/svd2rust/0.37.1/svd2rust/#peripheral-api
//! [next]: https://github.com/rust-embedded/svd2rust/blob/master/CHANGELOG.md#unreleased
//! [repository]: https://github.com/rust-embedded/svd2rust
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![no_std]
#![cfg_attr(docsrs, feature(doc_cfg))]
/// Number available in the NVIC for configuring priority
pub const NVIC_PRIO_BITS: u8 = 2;
#[allow(unused_imports)]
use generic::*;
/// Common register and bit access and modify traits
pub mod generic;
/// MOS 6510 on-chip I/O port at addresses 0x0000-0x0001: memory banking control
/// and Datassette lines. NOTE for svd2rust: baseAddress 0 makes the generated
/// RegisterBlock pointer null, which is unsound to dereference in Rust; either
/// exclude this peripheral from generation and hand-write the two accessors, or
/// patch the generated code. KERNAL reset writes D6510=0x2F, R6510=0x37.
pub type Cpuport = crate::Periph<cpuport::RegisterBlock, 0>;
impl core::fmt::Debug for Cpuport {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cpuport").finish()
    }
}
/// MOS 6510 on-chip I/O port at addresses 0x0000-0x0001: memory banking control
/// and Datassette lines. NOTE for svd2rust: baseAddress 0 makes the generated
/// RegisterBlock pointer null, which is unsound to dereference in Rust; either
/// exclude this peripheral from generation and hand-write the two accessors, or
/// patch the generated code. KERNAL reset writes D6510=0x2F, R6510=0x37.
pub mod cpuport;
/// VIC-II video interface (6567 NTSC / 6569 PAL) at 0xD000. Register image
/// repeats every 0x40 bytes through 0xD3FF. Unimplemented bits read as 1.
/// Addresses 0xD02F-0xD03F are not implemented on the 6567/6569: they read 0xFF
/// and writes have no effect. Caveat: on a C128 in C64 mode, the VIC-IIe does
/// implement 0xD02F (extended keyboard lines) and 0xD030 (2 MHz clock control)
/// at these addresses. The VIC addresses 16 KiB selected by CIA2.CI2PRA bits
/// 0-1. IRQ output drives the 6510 IRQ line.
pub type Vic = crate::Periph<vic::RegisterBlock, 0xd000>;
impl core::fmt::Debug for Vic {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Vic").finish()
    }
}
/// VIC-II video interface (6567 NTSC / 6569 PAL) at 0xD000. Register image
/// repeats every 0x40 bytes through 0xD3FF. Unimplemented bits read as 1.
/// Addresses 0xD02F-0xD03F are not implemented on the 6567/6569: they read 0xFF
/// and writes have no effect. Caveat: on a C128 in C64 mode, the VIC-IIe does
/// implement 0xD02F (extended keyboard lines) and 0xD030 (2 MHz clock control)
/// at these addresses. The VIC addresses 16 KiB selected by CIA2.CI2PRA bits
/// 0-1. IRQ output drives the 6510 IRQ line.
pub mod vic;
/// SID sound interface (6581/8580) at 0xD400. Register image repeats every 0x20
/// bytes through 0xD7FF. Voice and filter registers are write-only: reads
/// return a decaying copy of the last value written to any SID register, not
/// register contents (not modeled here). POTX/POTY digitize the RC discharge
/// time of the paddle inputs selected by CIA1.CIAPRA bits 6-7.
pub type Sid = crate::Periph<sid::RegisterBlock, 0xd400>;
impl core::fmt::Debug for Sid {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sid").finish()
    }
}
/// SID sound interface (6581/8580) at 0xD400. Register image repeats every 0x20
/// bytes through 0xD7FF. Voice and filter registers are write-only: reads
/// return a decaying copy of the last value written to any SID register, not
/// register contents (not modeled here). POTX/POTY digitize the RC discharge
/// time of the paddle inputs selected by CIA1.CIAPRA bits 6-7.
pub mod sid;
/// MOS 6526 CIA 1 at 0xDC00 (image repeats every 0x10 bytes through 0xDCFF):
/// keyboard matrix, joysticks, paddle select, Datassette read (FLAG1), timers.
/// IRQ output drives the 6510 IRQ line. TOD pin receives mains-derived 50/60
/// Hz. SP1/CNT1 are available on the user port.
pub type Cia1 = crate::Periph<cia1::RegisterBlock, 0xdc00>;
impl core::fmt::Debug for Cia1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cia1").finish()
    }
}
/// MOS 6526 CIA 1 at 0xDC00 (image repeats every 0x10 bytes through 0xDCFF):
/// keyboard matrix, joysticks, paddle select, Datassette read (FLAG1), timers.
/// IRQ output drives the 6510 IRQ line. TOD pin receives mains-derived 50/60
/// Hz. SP1/CNT1 are available on the user port.
pub mod cia1;
/// MOS 6526 CIA 2 at 0xDD00 (image repeats every 0x10 bytes through 0xDDFF):
/// VIC bank select, IEC serial bus, user port, RS-232. IRQ output drives the
/// 6510 NMI line (shared with the RESTORE key). FLAG2 is user port pin B
/// (RS-232 RXD). SP2/CNT2 are available on the user port. Defined in full
/// rather than derived from CIA1 so port and FLAG descriptions are correct;
/// register layout is identical to CIA1.
pub type Cia2 = crate::Periph<cia2::RegisterBlock, 0xdd00>;
impl core::fmt::Debug for Cia2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cia2").finish()
    }
}
/// MOS 6526 CIA 2 at 0xDD00 (image repeats every 0x10 bytes through 0xDDFF):
/// VIC bank select, IEC serial bus, user port, RS-232. IRQ output drives the
/// 6510 NMI line (shared with the RESTORE key). FLAG2 is user port pin B
/// (RS-232 RXD). SP2/CNT2 are available on the user port. Defined in full
/// rather than derived from CIA1 so port and FLAG descriptions are correct;
/// register layout is identical to CIA1.
pub mod cia2;
/// 1024 nybbles of static color RAM at 0xD800-0xDBFF, always visible when I/O
/// is banked in, independent of the VIC bank. Only bits 3:0 are stored; bits
/// 7:4 read as whatever the VIC last placed on its data bus (treat reads as
/// 4-bit). The first 1000 (0xD800-0xDBE7) back the 40x25 text/bitmap display;
/// the remaining 24 (0xDBE8-0xDBFF) are unused by the VIC but are real 4-bit
/// cells, usable as scratch.
pub type Colorram = crate::Periph<colorram::RegisterBlock, 0xd800>;
impl core::fmt::Debug for Colorram {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Colorram").finish()
    }
}
/// 1024 nybbles of static color RAM at 0xD800-0xDBFF, always visible when I/O
/// is banked in, independent of the VIC bank. Only bits 3:0 are stored; bits
/// 7:4 read as whatever the VIC last placed on its data bus (treat reads as
/// 4-bit). The first 1000 (0xD800-0xDBE7) back the 40x25 text/bitmap display;
/// the remaining 24 (0xDBE8-0xDBFF) are unused by the VIC but are real 4-bit
/// cells, usable as scratch.
pub mod colorram;
/// Expansion (cartridge) port chip-select window: the PLA asserts /IO1 for any
/// access in 0xDE00-0xDEFF. No on-board device responds; contents are defined
/// by the inserted cartridge. With nothing connected, reads return open bus (a
/// decaying copy of the last byte on the data bus) and writes have no effect.
pub type Io1 = crate::Periph<io1::RegisterBlock, 0xde00>;
impl core::fmt::Debug for Io1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Io1").finish()
    }
}
/// Expansion (cartridge) port chip-select window: the PLA asserts /IO1 for any
/// access in 0xDE00-0xDEFF. No on-board device responds; contents are defined
/// by the inserted cartridge. With nothing connected, reads return open bus (a
/// decaying copy of the last byte on the data bus) and writes have no effect.
pub mod io1;
/// Expansion (cartridge) port chip-select window: the PLA asserts /IO2 for any
/// access in 0xDF00-0xDFFF. Same behavior as IO1; commonly used by REU, disk
/// speeders and other cartridges.
pub type Io2 = crate::Periph<io2::RegisterBlock, 0xdf00>;
impl core::fmt::Debug for Io2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Io2").finish()
    }
}
/// Expansion (cartridge) port chip-select window: the PLA asserts /IO2 for any
/// access in 0xDF00-0xDFFF. Same behavior as IO1; commonly used by REU, disk
/// speeders and other cartridges.
pub mod io2;
#[unsafe(no_mangle)]
static mut DEVICE_PERIPHERALS: bool = false;
/// All the peripherals.
#[allow(non_snake_case)]
pub struct Peripherals {
    /// CPUPORT
    pub cpuport: Cpuport,
    /// VIC
    pub vic: Vic,
    /// SID
    pub sid: Sid,
    /// CIA1
    pub cia1: Cia1,
    /// CIA2
    pub cia2: Cia2,
    /// COLORRAM
    pub colorram: Colorram,
    /// IO1
    pub io1: Io1,
    /// IO2
    pub io2: Io2,
}
impl Peripherals {
    /// Returns all the peripherals *once*.
    #[cfg(feature = "critical-section")]
    #[inline]
    pub fn take() -> Option<Self> {
        critical_section::with(|_| {
            if unsafe { DEVICE_PERIPHERALS } {
                return None;
            }
            Some(unsafe { Peripherals::steal() })
        })
    }

    /// Unchecked version of `Peripherals::take`.
    ///
    /// # Safety
    ///
    /// Each of the returned peripherals must be used at most once.
    #[inline]
    pub unsafe fn steal() -> Self {
        DEVICE_PERIPHERALS = true;
        Peripherals { cpuport: Cpuport::steal(),
                      vic: Vic::steal(),
                      sid: Sid::steal(),
                      cia1: Cia1::steal(),
                      cia2: Cia2::steal(),
                      colorram: Colorram::steal(),
                      io1: Io1::steal(),
                      io2: Io2::steal() }
    }
}

pub mod bcd;
