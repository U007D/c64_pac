/// Register `CUTLO` writer
pub type W = crate::W<CutloSpec>;
/// Field `FILTER_CUTOFF_FREQ` writer - Filter cutoff bits 2:0
pub type FilterCutoffFreqW<'a, REG> = crate::FieldWriter<'a, REG, 3, u8, crate::Safe>;
impl core::fmt::Debug for crate::generic::Reg<CutloSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    /// Bits 0:2 - Filter cutoff bits 2:0
    #[inline(always)]
    pub fn filter_cutoff_freq(&mut self) -> FilterCutoffFreqW<'_, CutloSpec> {
        FilterCutoffFreqW::new(self, 0)
    }
}
/// Filter cutoff bits 2:0 (bits 3-7 unused)
///
/// You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cutlo::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct CutloSpec;
impl crate::RegisterSpec for CutloSpec {
    type Ux = u8;
}
/// `write(|w| ..)` method takes [`cutlo::W`](W) writer structure
impl crate::Writable for CutloSpec {
    type Safety = crate::Unsafe;
}
/// `reset()` method sets CUTLO to value 0
impl crate::Resettable for CutloSpec {}
