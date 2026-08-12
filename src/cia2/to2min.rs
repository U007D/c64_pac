/// Register `TO2MIN` reader
pub type R = crate::R<To2minSpec>;
/// Register `TO2MIN` writer
pub type W = crate::W<To2minSpec>;
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
        f.debug_struct("TO2MIN").field("minutes", &self.minutes()).finish()
    }
}
impl W {
    /// Bits 0:6 - Minutes 00-59, BCD
    #[inline(always)]
    pub fn minutes(&mut self) -> MinutesW<'_, To2minSpec> { MinutesW::new(self, 0) }
}
/// TOD minutes, BCD. SIDE-EFFECT: writes set the clock or the alarm per
/// CI2CRB.TOD_WRITE_MODE; the read value is frozen while the TO2HRS/TO2TEN
/// latch is held.
///
/// You can [`read`](crate::Reg::read) this register and get [`to2min::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`to2min::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct To2minSpec;
impl crate::RegisterSpec for To2minSpec {
    type Ux = u8;
}
/// `read()` method returns [`to2min::R`](R) reader structure
impl crate::Readable for To2minSpec {}
/// `write(|w| ..)` method takes [`to2min::W`](W) writer structure
impl crate::Writable for To2minSpec {
    type Safety = crate::Unsafe;
}
/// `reset()` method sets TO2MIN to value 0
impl crate::Resettable for To2minSpec {}
