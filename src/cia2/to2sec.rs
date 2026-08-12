/// Register `TO2SEC` reader
pub type R = crate::R<To2secSpec>;
/// Register `TO2SEC` writer
pub type W = crate::W<To2secSpec>;
/// Field `SECONDS` reader - Seconds 00-59, BCD
pub type SecondsR = crate::FieldReader;
/// Field `SECONDS` writer - Seconds 00-59, BCD
pub type SecondsW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    /// Bits 0:6 - Seconds 00-59, BCD
    #[inline(always)]
    pub fn seconds(&self) -> SecondsR { SecondsR::new(self.bits & 0x7f) }
}
impl W {
    /// Bits 0:6 - Seconds 00-59, BCD
    #[inline(always)]
    pub fn seconds(&mut self) -> SecondsW<'_, To2secSpec> { SecondsW::new(self, 0) }
}
/// TOD seconds, BCD. SIDE-EFFECT: writes set the clock or the alarm per
/// CI2CRB.TOD_WRITE_MODE; the read value is frozen while the TO2HRS/TO2TEN
/// latch is held.
///
/// You can [`read`](crate::Reg::read) this register and get [`to2sec::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`to2sec::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct To2secSpec;
impl crate::RegisterSpec for To2secSpec {
    type Ux = u8;
}
/// `read()` method returns [`to2sec::R`](R) reader structure
impl crate::Readable for To2secSpec {}
/// `write(|w| ..)` method takes [`to2sec::W`](W) writer structure
impl crate::Writable for To2secSpec {
    type Safety = crate::Unsafe;
}
/// `reset()` method sets TO2SEC to value 0
impl crate::Resettable for To2secSpec {}
