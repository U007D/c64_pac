/// Register `TODSEC` reader
pub type R = crate::R<TodsecSpec>;
/// Register `TODSEC` writer
pub type W = crate::W<TodsecSpec>;
/// Field `SECONDS` reader - Seconds 00-59, BCD
pub type SecondsR = crate::FieldReader;
/// Field `SECONDS` writer - Seconds 00-59, BCD
pub type SecondsW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    /// Bits 0:6 - Seconds 00-59, BCD
    #[inline(always)]
    pub fn seconds(&self) -> SecondsR { SecondsR::new(self.bits & 0x7f) }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TODSEC").field("seconds", &self.seconds()).finish()
    }
}
impl W {
    /// Bits 0:6 - Seconds 00-59, BCD
    #[inline(always)]
    pub fn seconds(&mut self) -> SecondsW<'_, TodsecSpec> { SecondsW::new(self, 0) }
}
/// TOD seconds, BCD. SIDE-EFFECT: writes set the clock or the alarm per
/// CIACRB.TOD_WRITE_MODE; the read value is frozen while the TODHRS/TODTEN
/// latch is held.
///
/// You can [`read`](crate::Reg::read) this register and get [`todsec::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`todsec::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct TodsecSpec;
impl crate::RegisterSpec for TodsecSpec {
    type Ux = u8;
}
/// `read()` method returns [`todsec::R`](R) reader structure
impl crate::Readable for TodsecSpec {}
/// `write(|w| ..)` method takes [`todsec::W`](W) writer structure
impl crate::Writable for TodsecSpec {
    type Safety = crate::Unsafe;
}
/// `reset()` method sets TODSEC to value 0
impl crate::Resettable for TodsecSpec {}
