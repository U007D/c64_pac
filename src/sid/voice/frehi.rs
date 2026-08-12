/// Register `FREHI` writer
pub type W = crate::W<FrehiSpec>;
/// Field `FREQUENCY` writer - Oscillator frequency, high 8 bits
pub type FrequencyW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl core::fmt::Debug for crate::generic::Reg<FrehiSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    /// Bits 0:7 - Oscillator frequency, high 8 bits
    #[inline(always)]
    pub fn frequency(&mut self) -> FrequencyW<'_, FrehiSpec> { FrequencyW::new(self, 0) }
}
/// Oscillator frequency bits 15:8
///
/// You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`frehi::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct FrehiSpec;
impl crate::RegisterSpec for FrehiSpec {
    type Ux = u8;
}
/// `write(|w| ..)` method takes [`frehi::W`](W) writer structure
impl crate::Writable for FrehiSpec {
    type Safety = crate::Unsafe;
}
/// `reset()` method sets FREHI to value 0
impl crate::Resettable for FrehiSpec {}
