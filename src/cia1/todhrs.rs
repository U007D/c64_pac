/// Register `TODHRS` reader
pub type R = crate::R<TodhrsSpec>;
/// Register `TODHRS` writer
pub type W = crate::W<TodhrsSpec>;
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
        f.debug_struct("TODHRS").field("hours", &self.hours()).field("pm", &self.pm()).finish()
    }
}
impl W {
    /// Bits 0:4 - Hours 01-12, BCD
    #[inline(always)]
    pub fn hours(&mut self) -> HoursW<'_, TodhrsSpec> { HoursW::new(self, 0) }

    /// Bit 7 - 1 = PM
    #[inline(always)]
    pub fn pm(&mut self) -> PmW<'_, TodhrsSpec> { PmW::new(self, 7) }
}
/// TOD hours, BCD, 12-hour with PM flag. SIDE-EFFECT: reading latches all four
/// TOD registers until TODTEN is read; writing (in clock mode) stops the clock
/// until TODTEN is written; CIACRB.TOD_WRITE_MODE selects whether writes set
/// the clock or the alarm.
///
/// You can [`read`](crate::Reg::read) this register and get [`todhrs::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`todhrs::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct TodhrsSpec;
impl crate::RegisterSpec for TodhrsSpec {
    type Ux = u8;
}
/// `read()` method returns [`todhrs::R`](R) reader structure
impl crate::Readable for TodhrsSpec {}
/// `write(|w| ..)` method takes [`todhrs::W`](W) writer structure
impl crate::Writable for TodhrsSpec {
    type Safety = crate::Unsafe;
}
/// `reset()` method sets TODHRS to value 0
impl crate::Resettable for TodhrsSpec {}
