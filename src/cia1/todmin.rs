/// Register `TODMIN` reader
pub type R = crate::R<TodminSpec>;
/// Register `TODMIN` writer
pub type W = crate::W<TodminSpec>;
/// Field `MINUTES` reader - Minutes 00-59, BCD
pub type MinutesR = crate::FieldReader;
/// Field `MINUTES` writer - Minutes 00-59, BCD
pub type MinutesW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    /// Bits 0:6 - Minutes 00-59, BCD
    #[inline(always)]
    pub fn minutes(&self) -> MinutesR { MinutesR::new(self.bits & 0x7f) }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TODMIN").field("minutes", &self.minutes()).finish()
    }
}
impl W {
    /// Bits 0:6 - Minutes 00-59, BCD
    #[inline(always)]
    pub fn minutes(&mut self) -> MinutesW<'_, TodminSpec> { MinutesW::new(self, 0) }
}
/// TOD minutes, BCD. SIDE-EFFECT: writes set the clock or the alarm per
/// CIACRB.TOD_WRITE_MODE; the read value is frozen while the TODHRS/TODTEN
/// latch is held.
///
/// You can [`read`](crate::Reg::read) this register and get [`todmin::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`todmin::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct TodminSpec;
impl crate::RegisterSpec for TodminSpec {
    type Ux = u8;
}
/// `read()` method returns [`todmin::R`](R) reader structure
impl crate::Readable for TodminSpec {}
/// `write(|w| ..)` method takes [`todmin::W`](W) writer structure
impl crate::Writable for TodminSpec {
    type Safety = crate::Unsafe;
}
/// `reset()` method sets TODMIN to value 0
impl crate::Resettable for TodminSpec {}
