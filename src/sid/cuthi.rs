/// Register `CUTHI` writer
pub type W = crate::W<CuthiSpec>;
/// Field `FILTER_CUTOFF_FREQ` writer - Filter cutoff frequency, high 8 bits
pub type FilterCutoffFreqW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl core::fmt::Debug for crate::generic::Reg<CuthiSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    /// Bits 0:7 - Filter cutoff frequency, high 8 bits
    #[inline(always)]
    pub fn filter_cutoff_freq(&mut self) -> FilterCutoffFreqW<'_, CuthiSpec> {
        FilterCutoffFreqW::new(self, 0)
    }
}
/// Filter cutoff bits 10:3
///
/// You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cuthi::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct CuthiSpec;
impl crate::RegisterSpec for CuthiSpec {
    type Ux = u8;
}
/// `write(|w| ..)` method takes [`cuthi::W`](W) writer structure
impl crate::Writable for CuthiSpec {
    type Safety = crate::Unsafe;
}
/// `reset()` method sets CUTHI to value 0
impl crate::Resettable for CuthiSpec {}
