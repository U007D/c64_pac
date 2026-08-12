#[repr(C)]
#[derive(Debug)]
/// Voice %s
#[doc(alias = "VOICE")]
pub struct Voice {
    frelo: Frelo,
    frehi: Frehi,
    pwlo: Pwlo,
    pwhi: Pwhi,
    vcreg: Vcreg,
    atdcy: Atdcy,
    surel: Surel,
}
impl Voice {
    /// 0x00 - Oscillator frequency bits 7:0
    #[inline(always)]
    pub const fn frelo(&self) -> &Frelo { &self.frelo }

    /// 0x01 - Oscillator frequency bits 15:8
    #[inline(always)]
    pub const fn frehi(&self) -> &Frehi { &self.frehi }

    /// 0x02 - Pulse width bits 7:0
    #[inline(always)]
    pub const fn pwlo(&self) -> &Pwlo { &self.pwlo }

    /// 0x03 - Pulse width bits 11:8 (bits 4-7 unused)
    #[inline(always)]
    pub const fn pwhi(&self) -> &Pwhi { &self.pwhi }

    /// 0x04 - Voice control
    #[inline(always)]
    pub const fn vcreg(&self) -> &Vcreg { &self.vcreg }

    /// 0x05 - Envelope attack/decay
    #[inline(always)]
    pub const fn atdcy(&self) -> &Atdcy { &self.atdcy }

    /// 0x06 - Envelope sustain/release
    #[inline(always)]
    pub const fn surel(&self) -> &Surel { &self.surel }
}
/// FRELO (w) register accessor: Oscillator frequency bits 7:0
///
/// You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`frelo::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
/// For information about available fields see [`mod@frelo`] module
#[doc(alias = "FRELO")]
pub type Frelo = crate::Reg<frelo::FreloSpec>;
/// Oscillator frequency bits 7:0
pub mod frelo;
/// FREHI (w) register accessor: Oscillator frequency bits 15:8
///
/// You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`frehi::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
/// For information about available fields see [`mod@frehi`] module
#[doc(alias = "FREHI")]
pub type Frehi = crate::Reg<frehi::FrehiSpec>;
/// Oscillator frequency bits 15:8
pub mod frehi;
/// PWLO (w) register accessor: Pulse width bits 7:0
///
/// You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwlo::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
/// For information about available fields see [`mod@pwlo`] module
#[doc(alias = "PWLO")]
pub type Pwlo = crate::Reg<pwlo::PwloSpec>;
/// Pulse width bits 7:0
pub mod pwlo;
/// PWHI (w) register accessor: Pulse width bits 11:8 (bits 4-7 unused)
///
/// You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwhi::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
/// For information about available fields see [`mod@pwhi`] module
#[doc(alias = "PWHI")]
pub type Pwhi = crate::Reg<pwhi::PwhiSpec>;
/// Pulse width bits 11:8 (bits 4-7 unused)
pub mod pwhi;
/// VCREG (w) register accessor: Voice control
///
/// You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vcreg::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
/// For information about available fields see [`mod@vcreg`] module
#[doc(alias = "VCREG")]
pub type Vcreg = crate::Reg<vcreg::VcregSpec>;
/// Voice control
pub mod vcreg;
/// ATDCY (w) register accessor: Envelope attack/decay
///
/// You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`atdcy::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
/// For information about available fields see [`mod@atdcy`] module
#[doc(alias = "ATDCY")]
pub type Atdcy = crate::Reg<atdcy::AtdcySpec>;
/// Envelope attack/decay
pub mod atdcy;
/// SUREL (w) register accessor: Envelope sustain/release
///
/// You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`surel::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
/// For information about available fields see [`mod@surel`] module
#[doc(alias = "SUREL")]
pub type Surel = crate::Reg<surel::SurelSpec>;
/// Envelope sustain/release
pub mod surel;
