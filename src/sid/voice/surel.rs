/// Register `SUREL` writer
pub type W = crate::W<SurelSpec>;
/// Field `RELEASE` writer - Release rate
pub type ReleaseW<'a, REG> = crate::FieldWriter<'a, REG, 4, u8, crate::Safe>;
/// Field `SUSTAIN` writer - Sustain level
pub type SustainW<'a, REG> = crate::FieldWriter<'a, REG, 4, u8, crate::Safe>;
impl core::fmt::Debug for crate::generic::Reg<SurelSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    /// Bits 0:3 - Release rate
    #[inline(always)]
    pub fn release(&mut self) -> ReleaseW<'_, SurelSpec> { ReleaseW::new(self, 0) }

    /// Bits 4:7 - Sustain level
    #[inline(always)]
    pub fn sustain(&mut self) -> SustainW<'_, SurelSpec> { SustainW::new(self, 4) }
}
/// Envelope sustain/release
///
/// You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`surel::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct SurelSpec;
impl crate::RegisterSpec for SurelSpec {
    type Ux = u8;
}
/// `write(|w| ..)` method takes [`surel::W`](W) writer structure
impl crate::Writable for SurelSpec {
    type Safety = crate::Unsafe;
}
/// `reset()` method sets SUREL to value 0
impl crate::Resettable for SurelSpec {}
