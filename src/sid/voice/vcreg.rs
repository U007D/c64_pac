/// Register `VCREG` writer
pub type W = crate::W<VcregSpec>;
/// Envelope gate: 0 starts release, 1 starts attack-decay-sustain
///
/// Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Start {
    /// 0: `0`
    StartR = 0,
    /// 1: `1`
    StartAds = 1,
}
impl From<Start> for bool {
    #[inline(always)]
    fn from(variant: Start) -> Self { variant as u8 != 0 }
}
/// Field `GATE` writer - Envelope gate: 0 starts release, 1 starts
/// attack-decay-sustain
pub type GateW<'a, REG> = crate::BitWriter<'a, REG, Start>;
impl<'a, REG> GateW<'a, REG> where REG: crate::Writable + crate::RegisterSpec, {
    /// `0`
    #[inline(always)]
    pub fn start_r(self) -> &'a mut crate::W<REG> { self.variant(Start::StartR) }

    /// `1`
    #[inline(always)]
    pub fn start_ads(self) -> &'a mut crate::W<REG> { self.variant(Start::StartAds) }
}
/// Hard-sync oscillator with the preceding voice
pub use crate::vic::scroly::Enable;
/// Field `SYNC` writer - Hard-sync oscillator with the preceding voice
pub use crate::vic::scroly::ScreenW as SyncW;
/// Field `RING` writer - Ring modulation with the preceding voice (triangle
/// output)
pub use crate::vic::scroly::ScreenW as RingW;
/// Reset and hold the oscillator (Disabled = held)
///
/// Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Test {
    /// 0: `0`
    Enabled = 0,
    /// 1: `1`
    Disabled = 1,
}
impl From<Test> for bool {
    #[inline(always)]
    fn from(variant: Test) -> Self { variant as u8 != 0 }
}
/// Field `TEST` writer - Reset and hold the oscillator (Disabled = held)
pub type TestW<'a, REG> = crate::BitWriter<'a, REG, Test>;
impl<'a, REG> TestW<'a, REG> where REG: crate::Writable + crate::RegisterSpec, {
    /// `0`
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> { self.variant(Test::Enabled) }

    /// `1`
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> { self.variant(Test::Disabled) }
}
/// Field `TRIANGLE` writer - Triangle waveform
pub use crate::vic::scroly::ScreenW as TriangleW;
/// Field `SAWTOOTH` writer - Sawtooth waveform
pub use crate::vic::scroly::ScreenW as SawtoothW;
/// Field `PULSE` writer - Pulse waveform
pub use crate::vic::scroly::ScreenW as PulseW;
/// Field `NOISE` writer - Noise waveform
pub use crate::vic::scroly::ScreenW as NoiseW;
impl core::fmt::Debug for crate::generic::Reg<VcregSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    /// Bit 0 - Envelope gate: 0 starts release, 1 starts attack-decay-sustain
    #[inline(always)]
    pub fn gate(&mut self) -> GateW<'_, VcregSpec> { GateW::new(self, 0) }

    /// Bit 1 - Hard-sync oscillator with the preceding voice
    #[inline(always)]
    pub fn sync(&mut self) -> SyncW<'_, VcregSpec> { SyncW::new(self, 1) }

    /// Bit 2 - Ring modulation with the preceding voice (triangle output)
    #[inline(always)]
    pub fn ring(&mut self) -> RingW<'_, VcregSpec> { RingW::new(self, 2) }

    /// Bit 3 - Reset and hold the oscillator (Disabled = held)
    #[inline(always)]
    pub fn test(&mut self) -> TestW<'_, VcregSpec> { TestW::new(self, 3) }

    /// Bit 4 - Triangle waveform
    #[inline(always)]
    pub fn triangle(&mut self) -> TriangleW<'_, VcregSpec> { TriangleW::new(self, 4) }

    /// Bit 5 - Sawtooth waveform
    #[inline(always)]
    pub fn sawtooth(&mut self) -> SawtoothW<'_, VcregSpec> { SawtoothW::new(self, 5) }

    /// Bit 6 - Pulse waveform
    #[inline(always)]
    pub fn pulse(&mut self) -> PulseW<'_, VcregSpec> { PulseW::new(self, 6) }

    /// Bit 7 - Noise waveform
    #[inline(always)]
    pub fn noise(&mut self) -> NoiseW<'_, VcregSpec> { NoiseW::new(self, 7) }
}
/// Voice control
///
/// You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vcreg::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct VcregSpec;
impl crate::RegisterSpec for VcregSpec {
    type Ux = u8;
}
/// `write(|w| ..)` method takes [`vcreg::W`](W) writer structure
impl crate::Writable for VcregSpec {
    type Safety = crate::Unsafe;
}
/// `reset()` method sets VCREG to value 0
impl crate::Resettable for VcregSpec {}
