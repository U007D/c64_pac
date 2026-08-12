/// Register `SIGVOL` writer
pub type W = crate::W<SigvolSpec>;
/// Field `VOL` writer - Master volume (0-15)
pub type VolW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
/// Low-pass filter output
pub use crate::vic::scroly::Enable;
/// Field `LOW_PASS` writer - Low-pass filter output
pub use crate::vic::scroly::ScreenW as LowPassW;
/// Field `BAND_PASS` writer - Band-pass filter output
pub use crate::vic::scroly::ScreenW as BandPassW;
/// Field `HIGH_PASS` writer - High-pass filter output
pub use crate::vic::scroly::ScreenW as HighPassW;
/// Voice 3 audio output (Disabled = disconnected)
///
/// Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Voice3 {
    /// 0: `0`
    Enabled = 0,
    /// 1: `1`
    Disabled = 1,
}
impl From<Voice3> for bool {
    #[inline(always)]
    fn from(variant: Voice3) -> Self { variant as u8 != 0 }
}
/// Field `VOICE3OFF` writer - Voice 3 audio output (Disabled = disconnected)
pub type Voice3offW<'a, REG> = crate::BitWriter<'a, REG, Voice3>;
impl<'a, REG> Voice3offW<'a, REG> where REG: crate::Writable + crate::RegisterSpec, {
    /// `0`
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> { self.variant(Voice3::Enabled) }

    /// `1`
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> { self.variant(Voice3::Disabled) }
}
impl W {
    /// Bits 0:3 - Master volume (0-15)
    #[inline(always)]
    pub fn vol(&mut self) -> VolW<'_, SigvolSpec> { VolW::new(self, 0) }

    /// Bit 4 - Low-pass filter output
    #[inline(always)]
    pub fn low_pass(&mut self) -> LowPassW<'_, SigvolSpec> { LowPassW::new(self, 4) }

    /// Bit 5 - Band-pass filter output
    #[inline(always)]
    pub fn band_pass(&mut self) -> BandPassW<'_, SigvolSpec> { BandPassW::new(self, 5) }

    /// Bit 6 - High-pass filter output
    #[inline(always)]
    pub fn high_pass(&mut self) -> HighPassW<'_, SigvolSpec> { HighPassW::new(self, 6) }

    /// Bit 7 - Voice 3 audio output (Disabled = disconnected)
    #[inline(always)]
    pub fn voice3off(&mut self) -> Voice3offW<'_, SigvolSpec> { Voice3offW::new(self, 7) }
}
/// Filter mode and master volume
///
/// You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sigvol::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct SigvolSpec;
impl crate::RegisterSpec for SigvolSpec {
    type Ux = u8;
}
/// `write(|w| ..)` method takes [`sigvol::W`](W) writer structure
impl crate::Writable for SigvolSpec {
    type Safety = crate::Unsafe;
}
/// `reset()` method sets SIGVOL to value 0
impl crate::Resettable for SigvolSpec {}
