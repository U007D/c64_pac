#[repr(C)]
#[derive(Debug)]
/// Register block
pub struct RegisterBlock {
    ciapra: Ciapra,
    ciaprb: Ciaprb,
    ciddra: Ciddra,
    ciddrb: Ciddrb,
    timalo: Timalo,
    timahi: Timahi,
    timblo: Timblo,
    timbhi: Timbhi,
    todten: Todten,
    todsec: Todsec,
    todmin: Todmin,
    todhrs: Todhrs,
    ciasdr: Ciasdr,
    _reserved_13_ciaicr: [u8; 0x01],
    ciacra: Ciacra,
    ciacrb: Ciacrb,
}
impl RegisterBlock {
    /// 0x00 - Port A. Doubles as keyboard-matrix column drive (outputs, active
    /// low) and joystick port 2 (bits 0-4, active low). Bits 6-7 select which
    /// control port's paddles reach SID POTX/POTY.
    #[inline(always)]
    pub const fn ciapra(&self) -> &Ciapra { &self.ciapra }

    /// 0x01 - Port B. Doubles as keyboard-matrix row sense (inputs, active low)
    /// and joystick port 1 (bits 0-4, active low; the fire line also triggers
    /// the light pen). PB6/PB7 can output timer A/B underflow.
    #[inline(always)]
    pub const fn ciaprb(&self) -> &Ciaprb { &self.ciaprb }

    /// 0x02 - Port A data direction (1 = output)
    #[inline(always)]
    pub const fn ciddra(&self) -> &Ciddra { &self.ciddra }

    /// 0x03 - Port B data direction (1 = output)
    #[inline(always)]
    pub const fn ciddrb(&self) -> &Ciddrb { &self.ciddrb }

    /// 0x04 - Timer A bits 7:0. Read: current count. Write: latch (reload
    /// value). HI/LO reads are not latched together, so a 16-bit read can tear
    /// across a decrement: read HI, LO, re-read HI and retry on change, or stop
    /// the timer.
    #[inline(always)]
    pub const fn timalo(&self) -> &Timalo { &self.timalo }

    /// 0x05 - Timer A bits 15:8. Read: current count. Write: latch; if the
    /// timer is stopped, writing also loads the counter.
    #[inline(always)]
    pub const fn timahi(&self) -> &Timahi { &self.timahi }

    /// 0x06 - Timer B bits 7:0 (as TALO)
    #[inline(always)]
    pub const fn timblo(&self) -> &Timblo { &self.timblo }

    /// 0x07 - Timer B bits 15:8 (as TAHI)
    #[inline(always)]
    pub const fn timbhi(&self) -> &Timbhi { &self.timbhi }

    /// 0x08 - TOD tenths of seconds, BCD. SIDE-EFFECT: reading releases the
    /// latch set by reading TODHRS; writing (in clock mode) restarts the clock;
    /// CIACRB.TOD_WRITE_MODE selects whether writes set the clock or the alarm.
    #[inline(always)]
    pub const fn todten(&self) -> &Todten { &self.todten }

    /// 0x09 - TOD seconds, BCD. SIDE-EFFECT: writes set the clock or the alarm
    /// per CIACRB.TOD_WRITE_MODE; the read value is frozen while the
    /// TODHRS/TODTEN latch is held.
    #[inline(always)]
    pub const fn todsec(&self) -> &Todsec { &self.todsec }

    /// 0x0a - TOD minutes, BCD. SIDE-EFFECT: writes set the clock or the alarm
    /// per CIACRB.TOD_WRITE_MODE; the read value is frozen while the
    /// TODHRS/TODTEN latch is held.
    #[inline(always)]
    pub const fn todmin(&self) -> &Todmin { &self.todmin }

    /// 0x0b - TOD hours, BCD, 12-hour with PM flag. SIDE-EFFECT: reading
    /// latches all four TOD registers until TODTEN is read; writing (in clock
    /// mode) stops the clock until TODTEN is written; CIACRB.TOD_WRITE_MODE
    /// selects whether writes set the clock or the alarm.
    #[inline(always)]
    pub const fn todhrs(&self) -> &Todhrs { &self.todhrs }

    /// 0x0c - Serial shift register on SP pin, clocked by CNT; direction set by
    /// CRA.SPMODE. Output rate is timer A underflow / 2.
    #[inline(always)]
    pub const fn ciasdr(&self) -> &Ciasdr { &self.ciasdr }

    /// 0x0d - Interrupt mask (write-only alternate view of 0xDC0D). Enable or
    /// disable individual interrupt sources without disturbing the others:
    /// `set()` the sources you want to change, then let `mode()` pick the
    /// direction — `mode().enabled()` enables every source you `set()`,
    /// `mode().disabled()` disables them. Sources left `clear()` (the default)
    /// are untouched either way. Read status via CIAICR_R. SIDE-EFFECT: this
    /// write changes which interrupts are enabled.
    #[inline(always)]
    pub const fn ciaicr_w(&self) -> &CiaicrW {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(13).cast() }
    }

    /// 0x0d - Interrupt data (read-only view of 0xDC0D, asserts IRQ): which
    /// sources have fired. Bit 7 (IR) reads Latched if any enabled source is
    /// pending. SIDE-EFFECT: reading clears all flags and releases the line, so
    /// capture everything from one read. Set the mask via CIAICR_W.
    #[inline(always)]
    pub const fn ciaicr_r(&self) -> &CiaicrR {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(13).cast() }
    }

    /// 0x0e - Control register A
    #[inline(always)]
    pub const fn ciacra(&self) -> &Ciacra { &self.ciacra }

    /// 0x0f - Control register B
    #[inline(always)]
    pub const fn ciacrb(&self) -> &Ciacrb { &self.ciacrb }
}
/// CIAPRA (rw) register accessor: Port A. Doubles as keyboard-matrix column
/// drive (outputs, active low) and joystick port 2 (bits 0-4, active low). Bits
/// 6-7 select which control port's paddles reach SID POTX/POTY.
///
/// You can [`read`](crate::Reg::read) this register and get [`ciapra::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ciapra::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
/// For information about available fields see [`mod@ciapra`] module
#[doc(alias = "CIAPRA")]
pub type Ciapra = crate::Reg<ciapra::CiapraSpec>;
/// Port A. Doubles as keyboard-matrix column drive (outputs, active low) and
/// joystick port 2 (bits 0-4, active low). Bits 6-7 select which control port's
/// paddles reach SID POTX/POTY.
pub mod ciapra;
/// CIAPRB (rw) register accessor: Port B. Doubles as keyboard-matrix row sense
/// (inputs, active low) and joystick port 1 (bits 0-4, active low; the fire
/// line also triggers the light pen). PB6/PB7 can output timer A/B underflow.
///
/// You can [`read`](crate::Reg::read) this register and get [`ciaprb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ciaprb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
/// For information about available fields see [`mod@ciaprb`] module
#[doc(alias = "CIAPRB")]
pub type Ciaprb = crate::Reg<ciaprb::CiaprbSpec>;
/// Port B. Doubles as keyboard-matrix row sense (inputs, active low) and
/// joystick port 1 (bits 0-4, active low; the fire line also triggers the light
/// pen). PB6/PB7 can output timer A/B underflow.
pub mod ciaprb;
/// CIDDRA (rw) register accessor: Port A data direction (1 = output)
///
/// You can [`read`](crate::Reg::read) this register and get [`ciddra::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ciddra::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
/// For information about available fields see [`mod@ciddra`] module
#[doc(alias = "CIDDRA")]
pub type Ciddra = crate::Reg<ciddra::CiddraSpec>;
/// Port A data direction (1 = output)
pub mod ciddra;
/// CIDDRB (rw) register accessor: Port B data direction (1 = output)
///
/// You can [`read`](crate::Reg::read) this register and get [`ciddrb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ciddrb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
/// For information about available fields see [`mod@ciddrb`] module
#[doc(alias = "CIDDRB")]
pub type Ciddrb = crate::Reg<ciddrb::CiddrbSpec>;
/// Port B data direction (1 = output)
pub mod ciddrb;
/// TIMALO (rw) register accessor: Timer A bits 7:0. Read: current count. Write:
/// latch (reload value). HI/LO reads are not latched together, so a 16-bit read
/// can tear across a decrement: read HI, LO, re-read HI and retry on change, or
/// stop the timer.
///
/// You can [`read`](crate::Reg::read) this register and get [`timalo::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timalo::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
/// For information about available fields see [`mod@timalo`] module
#[doc(alias = "TIMALO")]
pub type Timalo = crate::Reg<timalo::TimaloSpec>;
/// Timer A bits 7:0. Read: current count. Write: latch (reload value). HI/LO
/// reads are not latched together, so a 16-bit read can tear across a
/// decrement: read HI, LO, re-read HI and retry on change, or stop the timer.
pub mod timalo;
/// TIMAHI (rw) register accessor: Timer A bits 15:8. Read: current count.
/// Write: latch; if the timer is stopped, writing also loads the counter.
///
/// You can [`read`](crate::Reg::read) this register and get [`timahi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timahi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
/// For information about available fields see [`mod@timahi`] module
#[doc(alias = "TIMAHI")]
pub type Timahi = crate::Reg<timahi::TimahiSpec>;
/// Timer A bits 15:8. Read: current count. Write: latch; if the timer is
/// stopped, writing also loads the counter.
pub mod timahi;
/// TIMBLO (rw) register accessor: Timer B bits 7:0 (as TALO)
///
/// You can [`read`](crate::Reg::read) this register and get [`timblo::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timblo::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
/// For information about available fields see [`mod@timblo`] module
#[doc(alias = "TIMBLO")]
pub type Timblo = crate::Reg<timblo::TimbloSpec>;
/// Timer B bits 7:0 (as TALO)
pub mod timblo;
/// TIMBHI (rw) register accessor: Timer B bits 15:8 (as TAHI)
///
/// You can [`read`](crate::Reg::read) this register and get [`timbhi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timbhi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
/// For information about available fields see [`mod@timbhi`] module
#[doc(alias = "TIMBHI")]
pub type Timbhi = crate::Reg<timbhi::TimbhiSpec>;
/// Timer B bits 15:8 (as TAHI)
pub mod timbhi;
/// TODTEN (rw) register accessor: TOD tenths of seconds, BCD. SIDE-EFFECT:
/// reading releases the latch set by reading TODHRS; writing (in clock mode)
/// restarts the clock; CIACRB.TOD_WRITE_MODE selects whether writes set the
/// clock or the alarm.
///
/// You can [`read`](crate::Reg::read) this register and get [`todten::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`todten::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
/// For information about available fields see [`mod@todten`] module
#[doc(alias = "TODTEN")]
pub type Todten = crate::Reg<todten::TodtenSpec>;
/// TOD tenths of seconds, BCD. SIDE-EFFECT: reading releases the latch set by
/// reading TODHRS; writing (in clock mode) restarts the clock;
/// CIACRB.TOD_WRITE_MODE selects whether writes set the clock or the alarm.
pub mod todten;
/// TODSEC (rw) register accessor: TOD seconds, BCD. SIDE-EFFECT: writes set the
/// clock or the alarm per CIACRB.TOD_WRITE_MODE; the read value is frozen while
/// the TODHRS/TODTEN latch is held.
///
/// You can [`read`](crate::Reg::read) this register and get [`todsec::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`todsec::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
/// For information about available fields see [`mod@todsec`] module
#[doc(alias = "TODSEC")]
pub type Todsec = crate::Reg<todsec::TodsecSpec>;
/// TOD seconds, BCD. SIDE-EFFECT: writes set the clock or the alarm per
/// CIACRB.TOD_WRITE_MODE; the read value is frozen while the TODHRS/TODTEN
/// latch is held.
pub mod todsec;
/// TODMIN (rw) register accessor: TOD minutes, BCD. SIDE-EFFECT: writes set the
/// clock or the alarm per CIACRB.TOD_WRITE_MODE; the read value is frozen while
/// the TODHRS/TODTEN latch is held.
///
/// You can [`read`](crate::Reg::read) this register and get [`todmin::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`todmin::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
/// For information about available fields see [`mod@todmin`] module
#[doc(alias = "TODMIN")]
pub type Todmin = crate::Reg<todmin::TodminSpec>;
/// TOD minutes, BCD. SIDE-EFFECT: writes set the clock or the alarm per
/// CIACRB.TOD_WRITE_MODE; the read value is frozen while the TODHRS/TODTEN
/// latch is held.
pub mod todmin;
/// TODHRS (rw) register accessor: TOD hours, BCD, 12-hour with PM flag.
/// SIDE-EFFECT: reading latches all four TOD registers until TODTEN is read;
/// writing (in clock mode) stops the clock until TODTEN is written;
/// CIACRB.TOD_WRITE_MODE selects whether writes set the clock or the alarm.
///
/// You can [`read`](crate::Reg::read) this register and get [`todhrs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`todhrs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
/// For information about available fields see [`mod@todhrs`] module
#[doc(alias = "TODHRS")]
pub type Todhrs = crate::Reg<todhrs::TodhrsSpec>;
/// TOD hours, BCD, 12-hour with PM flag. SIDE-EFFECT: reading latches all four
/// TOD registers until TODTEN is read; writing (in clock mode) stops the clock
/// until TODTEN is written; CIACRB.TOD_WRITE_MODE selects whether writes set
/// the clock or the alarm.
pub mod todhrs;
/// CIASDR (rw) register accessor: Serial shift register on SP pin, clocked by
/// CNT; direction set by CRA.SPMODE. Output rate is timer A underflow / 2.
///
/// You can [`read`](crate::Reg::read) this register and get [`ciasdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ciasdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
/// For information about available fields see [`mod@ciasdr`] module
#[doc(alias = "CIASDR")]
pub type Ciasdr = crate::Reg<ciasdr::CiasdrSpec>;
/// Serial shift register on SP pin, clocked by CNT; direction set by
/// CRA.SPMODE. Output rate is timer A underflow / 2.
pub mod ciasdr;
/// CIAICR_R (r) register accessor: Interrupt data (read-only view of 0xDC0D,
/// asserts IRQ): which sources have fired. Bit 7 (IR) reads Latched if any
/// enabled source is pending. SIDE-EFFECT: reading clears all flags and
/// releases the line, so capture everything from one read. Set the mask via
/// CIAICR_W.
///
/// You can [`read`](crate::Reg::read) this register and get [`ciaicr_r::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
/// <div class="warning">The register is <b>cleared</b> (set to zero) following
/// a read operation.</div>
///
/// For information about available fields see [`mod@ciaicr_r`] module
#[doc(alias = "CIAICR_R")]
pub type CiaicrR = crate::Reg<ciaicr_r::CiaicrRSpec>;
/// Interrupt data (read-only view of 0xDC0D, asserts IRQ): which sources have
/// fired. Bit 7 (IR) reads Latched if any enabled source is pending.
/// SIDE-EFFECT: reading clears all flags and releases the line, so capture
/// everything from one read. Set the mask via CIAICR_W.
pub mod ciaicr_r;
/// CIAICR_W (w) register accessor: Interrupt mask (write-only alternate view of
/// 0xDC0D). Enable or disable individual interrupt sources without disturbing
/// the others: `set()` the sources you want to change, then let `mode()` pick
/// the direction — `mode().enabled()` enables every source you `set()`,
/// `mode().disabled()` disables them. Sources left `clear()` (the default) are
/// untouched either way. Read status via CIAICR_R. SIDE-EFFECT: this write
/// changes which interrupts are enabled.
///
/// You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ciaicr_w::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
/// For information about available fields see [`mod@ciaicr_w`] module
#[doc(alias = "CIAICR_W")]
pub type CiaicrW = crate::Reg<ciaicr_w::CiaicrWSpec>;
/// Interrupt mask (write-only alternate view of 0xDC0D). Enable or disable
/// individual interrupt sources without disturbing the others: `set()` the
/// sources you want to change, then let `mode()` pick the direction —
/// `mode().enabled()` enables every source you `set()`, `mode().disabled()`
/// disables them. Sources left `clear()` (the default) are untouched either
/// way. Read status via CIAICR_R. SIDE-EFFECT: this write changes which
/// interrupts are enabled.
pub mod ciaicr_w;
/// CIACRA (rw) register accessor: Control register A
///
/// You can [`read`](crate::Reg::read) this register and get [`ciacra::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ciacra::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
/// For information about available fields see [`mod@ciacra`] module
#[doc(alias = "CIACRA")]
pub type Ciacra = crate::Reg<ciacra::CiacraSpec>;
/// Control register A
pub mod ciacra;
/// CIACRB (rw) register accessor: Control register B
///
/// You can [`read`](crate::Reg::read) this register and get [`ciacrb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ciacrb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
/// For information about available fields see [`mod@ciacrb`] module
#[doc(alias = "CIACRB")]
pub type Ciacrb = crate::Reg<ciacrb::CiacrbSpec>;
/// Control register B
pub mod ciacrb;
