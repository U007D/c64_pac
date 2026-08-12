#[repr(C)]
/// Register block
pub struct RegisterBlock {
    ci2pra: Ci2pra,
    ci2prb: Ci2prb,
    c2ddra: C2ddra,
    c2ddrb: C2ddrb,
    ti2alo: Ti2alo,
    ti2ahi: Ti2ahi,
    ti2blo: Ti2blo,
    ti2bhi: Ti2bhi,
    to2ten: To2ten,
    to2sec: To2sec,
    to2min: To2min,
    to2hrs: To2hrs,
    ci2sdr: Ci2sdr,
    _reserved_13_ci2icr: [u8; 0x01],
    ci2cra: Ci2cra,
    ci2crb: Ci2crb,
}
impl RegisterBlock {
    /// 0x00 - Port A: VIC bank, IEC bus, RS-232 TXD. IEC outputs pass through
    /// inverting open-collector drivers: writing 1 pulls the bus line low.
    /// Inputs read the bus level directly.
    #[inline(always)]
    pub const fn ci2pra(&self) -> &Ci2pra { &self.ci2pra }

    /// 0x01 - Port B: user port PB0-PB7 (pins C-L). KERNAL RS-232 assigns RXD
    /// to PB0 (paralleled with FLAG2) and handshake lines to PB1-PB7.
    /// General-purpose GPIO when RS-232 is unused.
    #[inline(always)]
    pub const fn ci2prb(&self) -> &Ci2prb { &self.ci2prb }

    /// 0x02 - Port A data direction (1 = output). KERNAL sets 0x3F.
    #[inline(always)]
    pub const fn c2ddra(&self) -> &C2ddra { &self.c2ddra }

    /// 0x03 - Port B data direction (1 = output)
    #[inline(always)]
    pub const fn c2ddrb(&self) -> &C2ddrb { &self.c2ddrb }

    /// 0x04 - Timer A bits 7:0. Read: current count. Write: latch (reload
    /// value). HI/LO reads are not latched together, so a 16-bit read can tear
    /// across a decrement: read HI, LO, re-read HI and retry on change, or stop
    /// the timer.
    #[inline(always)]
    pub const fn ti2alo(&self) -> &Ti2alo { &self.ti2alo }

    /// 0x05 - Timer A bits 15:8. Read: current count. Write: latch; if the
    /// timer is stopped, writing also loads the counter.
    #[inline(always)]
    pub const fn ti2ahi(&self) -> &Ti2ahi { &self.ti2ahi }

    /// 0x06 - Timer B bits 7:0 (as TALO)
    #[inline(always)]
    pub const fn ti2blo(&self) -> &Ti2blo { &self.ti2blo }

    /// 0x07 - Timer B bits 15:8 (as TAHI)
    #[inline(always)]
    pub const fn ti2bhi(&self) -> &Ti2bhi { &self.ti2bhi }

    /// 0x08 - TOD tenths of seconds, BCD. SIDE-EFFECT: reading releases the
    /// latch set by reading TO2HRS; writing (in clock mode) restarts the clock;
    /// CI2CRB.TOD_WRITE_MODE selects clock vs alarm. Same behavior as
    /// CIA1.TODTEN.
    #[inline(always)]
    pub const fn to2ten(&self) -> &To2ten { &self.to2ten }

    /// 0x09 - TOD seconds, BCD. SIDE-EFFECT: writes set the clock or the alarm
    /// per CI2CRB.TOD_WRITE_MODE; the read value is frozen while the
    /// TO2HRS/TO2TEN latch is held.
    #[inline(always)]
    pub const fn to2sec(&self) -> &To2sec { &self.to2sec }

    /// 0x0a - TOD minutes, BCD. SIDE-EFFECT: writes set the clock or the alarm
    /// per CI2CRB.TOD_WRITE_MODE; the read value is frozen while the
    /// TO2HRS/TO2TEN latch is held.
    #[inline(always)]
    pub const fn to2min(&self) -> &To2min { &self.to2min }

    /// 0x0b - TOD hours, BCD, 12-hour with PM flag. SIDE-EFFECT: reading
    /// latches all four CIA2 TOD registers until TO2TEN is read; writing (in
    /// clock mode) stops the clock until TO2TEN is written;
    /// CI2CRB.TOD_WRITE_MODE selects clock vs alarm. Same behavior as
    /// CIA1.TODHRS.
    #[inline(always)]
    pub const fn to2hrs(&self) -> &To2hrs { &self.to2hrs }

    /// 0x0c - Serial shift register on SP2, clocked by CNT2; direction set by
    /// CRA.SPMODE
    #[inline(always)]
    pub const fn ci2sdr(&self) -> &Ci2sdr { &self.ci2sdr }

    /// 0x0d - Interrupt mask (write-only alternate view of 0xDD0D). Bit 7
    /// (MODE) is the set/clear selector: with MODE enabled the source bits you
    /// enable are turned on, with MODE disabled they are turned off; bits left
    /// 0 are unchanged. Read status via CI2ICR_R. SIDE-EFFECT: this write
    /// changes which interrupts are enabled.
    #[inline(always)]
    pub const fn ci2icr_w(&self) -> &Ci2icrW {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(13).cast() }
    }

    /// 0x0d - Interrupt data (read-only view of 0xDD0D, asserts NMI): which
    /// sources have fired. Bit 7 (IR) reads Latched if any enabled source is
    /// pending. SIDE-EFFECT: reading clears all flags and releases the line, so
    /// capture everything from one read. Set the mask via CI2ICR_W.
    #[inline(always)]
    pub const fn ci2icr_r(&self) -> &Ci2icrR {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(13).cast() }
    }

    /// 0x0e - Control register A (same layout as CIA1.CRA; the timer-output
    /// bits act on this CIA's PB6)
    #[inline(always)]
    pub const fn ci2cra(&self) -> &Ci2cra { &self.ci2cra }

    /// 0x0f - Control register B (same layout as CIA1.CRB; the timer-output
    /// bits act on this CIA's PB7)
    #[inline(always)]
    pub const fn ci2crb(&self) -> &Ci2crb { &self.ci2crb }
}
/// CI2PRA (rw) register accessor: Port A: VIC bank, IEC bus, RS-232 TXD. IEC
/// outputs pass through inverting open-collector drivers: writing 1 pulls the
/// bus line low. Inputs read the bus level directly.
///
/// You can [`read`](crate::Reg::read) this register and get [`ci2pra::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ci2pra::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
/// For information about available fields see [`mod@ci2pra`] module
#[doc(alias = "CI2PRA")]
pub type Ci2pra = crate::Reg<ci2pra::Ci2praSpec>;
/// Port A: VIC bank, IEC bus, RS-232 TXD. IEC outputs pass through inverting
/// open-collector drivers: writing 1 pulls the bus line low. Inputs read the
/// bus level directly.
pub mod ci2pra;
/// CI2PRB (rw) register accessor: Port B: user port PB0-PB7 (pins C-L). KERNAL
/// RS-232 assigns RXD to PB0 (paralleled with FLAG2) and handshake lines to
/// PB1-PB7. General-purpose GPIO when RS-232 is unused.
///
/// You can [`read`](crate::Reg::read) this register and get [`ci2prb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ci2prb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
/// For information about available fields see [`mod@ci2prb`] module
#[doc(alias = "CI2PRB")]
pub type Ci2prb = crate::Reg<ci2prb::Ci2prbSpec>;
/// Port B: user port PB0-PB7 (pins C-L). KERNAL RS-232 assigns RXD to PB0
/// (paralleled with FLAG2) and handshake lines to PB1-PB7. General-purpose GPIO
/// when RS-232 is unused.
pub mod ci2prb;
/// C2DDRA (rw) register accessor: Port A data direction (1 = output). KERNAL
/// sets 0x3F.
///
/// You can [`read`](crate::Reg::read) this register and get [`c2ddra::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c2ddra::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
/// For information about available fields see [`mod@c2ddra`] module
#[doc(alias = "C2DDRA")]
pub type C2ddra = crate::Reg<c2ddra::C2ddraSpec>;
/// Port A data direction (1 = output). KERNAL sets 0x3F.
pub mod c2ddra;
/// C2DDRB (rw) register accessor: Port B data direction (1 = output)
///
/// You can [`read`](crate::Reg::read) this register and get [`c2ddrb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c2ddrb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
/// For information about available fields see [`mod@c2ddrb`] module
#[doc(alias = "C2DDRB")]
pub type C2ddrb = crate::Reg<c2ddrb::C2ddrbSpec>;
/// Port B data direction (1 = output)
pub mod c2ddrb;
/// TI2ALO (rw) register accessor: Timer A bits 7:0. Read: current count. Write:
/// latch (reload value). HI/LO reads are not latched together, so a 16-bit read
/// can tear across a decrement: read HI, LO, re-read HI and retry on change, or
/// stop the timer.
///
/// You can [`read`](crate::Reg::read) this register and get [`ti2alo::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ti2alo::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
/// For information about available fields see [`mod@ti2alo`] module
#[doc(alias = "TI2ALO")]
pub type Ti2alo = crate::Reg<ti2alo::Ti2aloSpec>;
/// Timer A bits 7:0. Read: current count. Write: latch (reload value). HI/LO
/// reads are not latched together, so a 16-bit read can tear across a
/// decrement: read HI, LO, re-read HI and retry on change, or stop the timer.
pub mod ti2alo;
/// TI2AHI (rw) register accessor: Timer A bits 15:8. Read: current count.
/// Write: latch; if the timer is stopped, writing also loads the counter.
///
/// You can [`read`](crate::Reg::read) this register and get [`ti2ahi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ti2ahi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
/// For information about available fields see [`mod@ti2ahi`] module
#[doc(alias = "TI2AHI")]
pub type Ti2ahi = crate::Reg<ti2ahi::Ti2ahiSpec>;
/// Timer A bits 15:8. Read: current count. Write: latch; if the timer is
/// stopped, writing also loads the counter.
pub mod ti2ahi;
/// TI2BLO (rw) register accessor: Timer B bits 7:0 (as TALO)
///
/// You can [`read`](crate::Reg::read) this register and get [`ti2blo::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ti2blo::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
/// For information about available fields see [`mod@ti2blo`] module
#[doc(alias = "TI2BLO")]
pub type Ti2blo = crate::Reg<ti2blo::Ti2bloSpec>;
/// Timer B bits 7:0 (as TALO)
pub mod ti2blo;
/// TI2BHI (rw) register accessor: Timer B bits 15:8 (as TAHI)
///
/// You can [`read`](crate::Reg::read) this register and get [`ti2bhi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ti2bhi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
/// For information about available fields see [`mod@ti2bhi`] module
#[doc(alias = "TI2BHI")]
pub type Ti2bhi = crate::Reg<ti2bhi::Ti2bhiSpec>;
/// Timer B bits 15:8 (as TAHI)
pub mod ti2bhi;
/// TO2TEN (rw) register accessor: TOD tenths of seconds, BCD. SIDE-EFFECT:
/// reading releases the latch set by reading TO2HRS; writing (in clock mode)
/// restarts the clock; CI2CRB.TOD_WRITE_MODE selects clock vs alarm. Same
/// behavior as CIA1.TODTEN.
///
/// You can [`read`](crate::Reg::read) this register and get [`to2ten::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`to2ten::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
/// For information about available fields see [`mod@to2ten`] module
#[doc(alias = "TO2TEN")]
pub type To2ten = crate::Reg<to2ten::To2tenSpec>;
/// TOD tenths of seconds, BCD. SIDE-EFFECT: reading releases the latch set by
/// reading TO2HRS; writing (in clock mode) restarts the clock;
/// CI2CRB.TOD_WRITE_MODE selects clock vs alarm. Same behavior as CIA1.TODTEN.
pub mod to2ten;
/// TO2SEC (rw) register accessor: TOD seconds, BCD. SIDE-EFFECT: writes set the
/// clock or the alarm per CI2CRB.TOD_WRITE_MODE; the read value is frozen while
/// the TO2HRS/TO2TEN latch is held.
///
/// You can [`read`](crate::Reg::read) this register and get [`to2sec::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`to2sec::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
/// For information about available fields see [`mod@to2sec`] module
#[doc(alias = "TO2SEC")]
pub type To2sec = crate::Reg<to2sec::To2secSpec>;
/// TOD seconds, BCD. SIDE-EFFECT: writes set the clock or the alarm per
/// CI2CRB.TOD_WRITE_MODE; the read value is frozen while the TO2HRS/TO2TEN
/// latch is held.
pub mod to2sec;
/// TO2MIN (rw) register accessor: TOD minutes, BCD. SIDE-EFFECT: writes set the
/// clock or the alarm per CI2CRB.TOD_WRITE_MODE; the read value is frozen while
/// the TO2HRS/TO2TEN latch is held.
///
/// You can [`read`](crate::Reg::read) this register and get [`to2min::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`to2min::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
/// For information about available fields see [`mod@to2min`] module
#[doc(alias = "TO2MIN")]
pub type To2min = crate::Reg<to2min::To2minSpec>;
/// TOD minutes, BCD. SIDE-EFFECT: writes set the clock or the alarm per
/// CI2CRB.TOD_WRITE_MODE; the read value is frozen while the TO2HRS/TO2TEN
/// latch is held.
pub mod to2min;
/// TO2HRS (rw) register accessor: TOD hours, BCD, 12-hour with PM flag.
/// SIDE-EFFECT: reading latches all four CIA2 TOD registers until TO2TEN is
/// read; writing (in clock mode) stops the clock until TO2TEN is written;
/// CI2CRB.TOD_WRITE_MODE selects clock vs alarm. Same behavior as CIA1.TODHRS.
///
/// You can [`read`](crate::Reg::read) this register and get [`to2hrs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`to2hrs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
/// For information about available fields see [`mod@to2hrs`] module
#[doc(alias = "TO2HRS")]
pub type To2hrs = crate::Reg<to2hrs::To2hrsSpec>;
/// TOD hours, BCD, 12-hour with PM flag. SIDE-EFFECT: reading latches all four
/// CIA2 TOD registers until TO2TEN is read; writing (in clock mode) stops the
/// clock until TO2TEN is written; CI2CRB.TOD_WRITE_MODE selects clock vs alarm.
/// Same behavior as CIA1.TODHRS.
pub mod to2hrs;
/// CI2SDR (rw) register accessor: Serial shift register on SP2, clocked by
/// CNT2; direction set by CRA.SPMODE
///
/// You can [`read`](crate::Reg::read) this register and get [`ci2sdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ci2sdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
/// For information about available fields see [`mod@ci2sdr`] module
#[doc(alias = "CI2SDR")]
pub type Ci2sdr = crate::Reg<ci2sdr::Ci2sdrSpec>;
/// Serial shift register on SP2, clocked by CNT2; direction set by CRA.SPMODE
pub mod ci2sdr;
/// CI2ICR_R (r) register accessor: Interrupt data (read-only view of 0xDD0D,
/// asserts NMI): which sources have fired. Bit 7 (IR) reads Latched if any
/// enabled source is pending. SIDE-EFFECT: reading clears all flags and
/// releases the line, so capture everything from one read. Set the mask via
/// CI2ICR_W.
///
/// You can [`read`](crate::Reg::read) this register and get [`ci2icr_r::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
/// <div class="warning">The register is <b>cleared</b> (set to zero) following
/// a read operation.</div>
///
/// For information about available fields see [`mod@ci2icr_r`] module
#[doc(alias = "CI2ICR_R")]
pub type Ci2icrR = crate::Reg<ci2icr_r::Ci2icrRSpec>;
/// Interrupt data (read-only view of 0xDD0D, asserts NMI): which sources have
/// fired. Bit 7 (IR) reads Latched if any enabled source is pending.
/// SIDE-EFFECT: reading clears all flags and releases the line, so capture
/// everything from one read. Set the mask via CI2ICR_W.
pub mod ci2icr_r;
/// CI2ICR_W (w) register accessor: Interrupt mask (write-only alternate view of
/// 0xDD0D). Bit 7 (MODE) is the set/clear selector: with MODE enabled the
/// source bits you enable are turned on, with MODE disabled they are turned
/// off; bits left 0 are unchanged. Read status via CI2ICR_R. SIDE-EFFECT: this
/// write changes which interrupts are enabled.
///
/// You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ci2icr_w::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
/// For information about available fields see [`mod@ci2icr_w`] module
#[doc(alias = "CI2ICR_W")]
pub type Ci2icrW = crate::Reg<ci2icr_w::Ci2icrWSpec>;
/// Interrupt mask (write-only alternate view of 0xDD0D). Bit 7 (MODE) is the
/// set/clear selector: with MODE enabled the source bits you enable are turned
/// on, with MODE disabled they are turned off; bits left 0 are unchanged. Read
/// status via CI2ICR_R. SIDE-EFFECT: this write changes which interrupts are
/// enabled.
pub mod ci2icr_w;
/// CI2CRA (rw) register accessor: Control register A (same layout as CIA1.CRA;
/// the timer-output bits act on this CIA's PB6)
///
/// You can [`read`](crate::Reg::read) this register and get [`ci2cra::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ci2cra::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
/// For information about available fields see [`mod@ci2cra`] module
#[doc(alias = "CI2CRA")]
pub type Ci2cra = crate::Reg<ci2cra::Ci2craSpec>;
/// Control register A (same layout as CIA1.CRA; the timer-output bits act on
/// this CIA's PB6)
pub mod ci2cra;
/// CI2CRB (rw) register accessor: Control register B (same layout as CIA1.CRB;
/// the timer-output bits act on this CIA's PB7)
///
/// You can [`read`](crate::Reg::read) this register and get [`ci2crb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ci2crb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
/// For information about available fields see [`mod@ci2crb`] module
#[doc(alias = "CI2CRB")]
pub type Ci2crb = crate::Reg<ci2crb::Ci2crbSpec>;
/// Control register B (same layout as CIA1.CRB; the timer-output bits act on
/// this CIA's PB7)
pub mod ci2crb;
