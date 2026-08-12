/// Register `TO2TEN` reader
pub type R = crate::R<To2tenSpec>;
/// Register `TO2TEN` writer
pub type W = crate::W<To2tenSpec>;
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
    pub fn tenths(&mut self) -> TenthsW<'_, To2tenSpec> { TenthsW::new(self, 0) }
}
/// TOD tenths of seconds, BCD. SIDE-EFFECT: reading releases the latch set by
/// reading TO2HRS; writing (in clock mode) restarts the clock;
/// CI2CRB.TOD_WRITE_MODE selects clock vs alarm. Same behavior as CIA1.TODTEN.
///
/// You can [`read`](crate::Reg::read) this register and get [`to2ten::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`to2ten::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct To2tenSpec;
impl crate::RegisterSpec for To2tenSpec {
    type Ux = u8;
}
/// `read()` method returns [`to2ten::R`](R) reader structure
impl crate::Readable for To2tenSpec {}
/// `write(|w| ..)` method takes [`to2ten::W`](W) writer structure
impl crate::Writable for To2tenSpec {
    type Safety = crate::Unsafe;
}
/// `reset()` method sets TO2TEN to value 0
impl crate::Resettable for To2tenSpec {}
