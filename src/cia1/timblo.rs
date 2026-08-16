/// Register `TIMBLO` reader
pub type R = crate::R<TimbloSpec>;
/// Register `TIMBLO` writer
pub type W = crate::W<TimbloSpec>;
/// Field `TIMER` reader - Timer B, low 8 bits (read: counter; write: latch)
pub type TimerR = crate::FieldReader;
/// Field `TIMER` writer - Timer B, low 8 bits (read: counter; write: latch)
pub type TimerW<'a, REG> = crate::FieldWriter<'a, REG, 8, u8, crate::Safe>;
impl R {
    /// Bits 0:7 - Timer B, low 8 bits (read: counter; write: latch)
    #[inline(always)]
    pub fn timer(&self) -> TimerR { TimerR::new(self.bits) }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIMBLO").field("timer", &self.timer()).finish()
    }
}
impl W {
    /// Bits 0:7 - Timer B, low 8 bits (read: counter; write: latch)
    #[inline(always)]
    pub fn timer(&mut self) -> TimerW<'_, TimbloSpec> { TimerW::new(self, 0) }
}
/// Timer B bits 7:0 (as TALO)
///
/// You can [`read`](crate::Reg::read) this register and get [`timblo::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timblo::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct TimbloSpec;
impl crate::RegisterSpec for TimbloSpec {
    type Ux = u8;
}
/// `read()` method returns [`timblo::R`](R) reader structure
impl crate::Readable for TimbloSpec {}
/// `write(|w| ..)` method takes [`timblo::W`](W) writer structure
impl crate::Writable for TimbloSpec {
    type Safety = crate::Safe;
}
/// `reset()` method sets TIMBLO to value 0
impl crate::Resettable for TimbloSpec {}
