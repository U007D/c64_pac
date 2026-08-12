/// Register `RESON` writer
pub type W = crate::W<ResonSpec>;
/// Route voice 1 through the filter
pub use crate::vic::scroly::Enable;
/// Field `FILT_1` writer - Route voice 1 through the filter
pub use crate::vic::scroly::ScreenW as Filt1W;
/// Field `FILT_2` writer - Route voice 2 through the filter
pub use crate::vic::scroly::ScreenW as Filt2W;
/// Field `FILT_3` writer - Route voice 3 through the filter
pub use crate::vic::scroly::ScreenW as Filt3W;
/// Field `FILT_EXT` writer - Route external input through the filter
pub use crate::vic::scroly::ScreenW as FiltExtW;
/// Field `FILT_RES` writer - Filter resonance (0-15)
pub type FiltResW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl core::fmt::Debug for crate::generic::Reg<ResonSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    /// Bit 0 - Route voice 1 through the filter
    #[inline(always)]
    pub fn filt_1(&mut self) -> Filt1W<'_, ResonSpec> { Filt1W::new(self, 0) }

    /// Bit 1 - Route voice 2 through the filter
    #[inline(always)]
    pub fn filt_2(&mut self) -> Filt2W<'_, ResonSpec> { Filt2W::new(self, 1) }

    /// Bit 2 - Route voice 3 through the filter
    #[inline(always)]
    pub fn filt_3(&mut self) -> Filt3W<'_, ResonSpec> { Filt3W::new(self, 2) }

    /// Bit 3 - Route external input through the filter
    #[inline(always)]
    pub fn filt_ext(&mut self) -> FiltExtW<'_, ResonSpec> { FiltExtW::new(self, 3) }

    /// Bits 4:7 - Filter resonance (0-15)
    #[inline(always)]
    pub fn filt_res(&mut self) -> FiltResW<'_, ResonSpec> { FiltResW::new(self, 4) }
}
/// Filter resonance and routing
///
/// You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reson::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct ResonSpec;
impl crate::RegisterSpec for ResonSpec {
    type Ux = u8;
}
/// `write(|w| ..)` method takes [`reson::W`](W) writer structure
impl crate::Writable for ResonSpec {
    type Safety = crate::Unsafe;
}
/// `reset()` method sets RESON to value 0
impl crate::Resettable for ResonSpec {}
