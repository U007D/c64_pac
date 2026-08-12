/// Register `TODTEN` reader
pub type R = crate::R<TodtenSpec>;
/// Register `TODTEN` writer
pub type W = crate::W<TodtenSpec>;
/// Field `TENTHS` reader - Tenths, BCD
pub type TenthsR = crate::FieldReader;
/// Field `TENTHS` writer - Tenths, BCD
pub type TenthsW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    /// Bits 0:3 - Tenths, BCD
    #[inline(always)]
    pub fn tenths(&self) -> TenthsR { TenthsR::new(self.bits & 0x0f) }
}
impl W {
    /// Bits 0:3 - Tenths, BCD
    #[inline(always)]
    pub fn tenths(&mut self) -> TenthsW<'_, TodtenSpec> { TenthsW::new(self, 0) }
}
/// TOD tenths of seconds, BCD. SIDE-EFFECT: reading releases the latch set by
/// reading TODHRS; writing (in clock mode) restarts the clock;
/// CIACRB.TOD_WRITE_MODE selects whether writes set the clock or the alarm.
///
/// You can [`read`](crate::Reg::read) this register and get [`todten::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`todten::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct TodtenSpec;
impl crate::RegisterSpec for TodtenSpec {
    type Ux = u8;
}
/// `read()` method returns [`todten::R`](R) reader structure
impl crate::Readable for TodtenSpec {}
/// `write(|w| ..)` method takes [`todten::W`](W) writer structure
impl crate::Writable for TodtenSpec {
    type Safety = crate::Unsafe;
}
/// `reset()` method sets TODTEN to value 0
impl crate::Resettable for TodtenSpec {}
