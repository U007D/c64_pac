/// Register `FRELO` writer
pub type W = crate::W<FreloSpec>;
/// Field `FREQUENCY` writer - Oscillator frequency, low 8 bits
pub type FrequencyW<'a, REG> = crate::FieldWriter<'a, REG, 8, u8, crate::Safe>;
impl core::fmt::Debug for crate::generic::Reg<FreloSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    /// Bits 0:7 - Oscillator frequency, low 8 bits
    #[inline(always)]
    pub fn frequency(&mut self) -> FrequencyW<'_, FreloSpec> { FrequencyW::new(self, 0) }
}
/// Oscillator frequency bits 7:0
///
/// You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`frelo::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct FreloSpec;
impl crate::RegisterSpec for FreloSpec {
    type Ux = u8;
}
/// `write(|w| ..)` method takes [`frelo::W`](W) writer structure
impl crate::Writable for FreloSpec {
    type Safety = crate::Safe;
}
/// `reset()` method sets FRELO to value 0
impl crate::Resettable for FreloSpec {}
