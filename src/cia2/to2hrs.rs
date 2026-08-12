/// Register `TO2HRS` reader
pub type R = crate::R<To2hrsSpec>;
/// Register `TO2HRS` writer
pub type W = crate::W<To2hrsSpec>;
/// Field `HOURS` reader - Hours 01-12, BCD
pub type HoursR = crate::FieldReader;
/// Field `HOURS` writer - Hours 01-12, BCD
pub type HoursW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
/// 1 = PM
///
/// Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Meridiem {
    /// 0: `0`
    Am = 0,
    /// 1: `1`
    Pm = 1,
}
impl From<Meridiem> for bool {
    #[inline(always)]
    fn from(variant: Meridiem) -> Self { variant as u8 != 0 }
}
/// Field `PM` reader - 1 = PM
pub type PmR = crate::BitReader<Meridiem>;
impl PmR {
    /// Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Meridiem {
        match self.bits {
            false => Meridiem::Am,
            true => Meridiem::Pm,
        }
    }

    /// `0`
    #[inline(always)]
    pub fn is_am(&self) -> bool { *self == Meridiem::Am }

    /// `1`
    #[inline(always)]
    pub fn is_pm(&self) -> bool { *self == Meridiem::Pm }
}
/// Field `PM` writer - 1 = PM
pub type PmW<'a, REG> = crate::BitWriter<'a, REG, Meridiem>;
impl<'a, REG> PmW<'a, REG> where REG: crate::Writable + crate::RegisterSpec, {
    /// `0`
    #[inline(always)]
    pub fn am(self) -> &'a mut crate::W<REG> { self.variant(Meridiem::Am) }

    /// `1`
    #[inline(always)]
    pub fn pm(self) -> &'a mut crate::W<REG> { self.variant(Meridiem::Pm) }
}
impl R {
    /// Bits 0:4 - Hours 01-12, BCD
    #[inline(always)]
    pub fn hours(&self) -> HoursR { HoursR::new(self.bits & 0x1f) }

    /// Bit 7 - 1 = PM
    #[inline(always)]
    pub fn pm(&self) -> PmR { PmR::new(((self.bits >> 7) & 1) != 0) }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TO2HRS").field("hours", &self.hours()).field("pm", &self.pm()).finish()
    }
}
impl W {
    /// Bits 0:4 - Hours 01-12, BCD
    #[inline(always)]
    pub fn hours(&mut self) -> HoursW<'_, To2hrsSpec> { HoursW::new(self, 0) }

    /// Bit 7 - 1 = PM
    #[inline(always)]
    pub fn pm(&mut self) -> PmW<'_, To2hrsSpec> { PmW::new(self, 7) }
}
/// TOD hours, BCD, 12-hour with PM flag. SIDE-EFFECT: reading latches all four
/// CIA2 TOD registers until TO2TEN is read; writing (in clock mode) stops the
/// clock until TO2TEN is written; CI2CRB.TOD_WRITE_MODE selects clock vs alarm.
/// Same behavior as CIA1.TODHRS.
///
/// You can [`read`](crate::Reg::read) this register and get [`to2hrs::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`to2hrs::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct To2hrsSpec;
impl crate::RegisterSpec for To2hrsSpec {
    type Ux = u8;
}
/// `read()` method returns [`to2hrs::R`](R) reader structure
impl crate::Readable for To2hrsSpec {}
/// `write(|w| ..)` method takes [`to2hrs::W`](W) writer structure
impl crate::Writable for To2hrsSpec {
    type Safety = crate::Unsafe;
}
/// `reset()` method sets TO2HRS to value 0
impl crate::Resettable for To2hrsSpec {}
