/// Register `TI2BLO` reader
pub type R = crate::R<Ti2bloSpec>;
/// Register `TI2BLO` writer
pub type W = crate::W<Ti2bloSpec>;
/// Field `TIMER` reader - Timer B, low 8 bits (read: counter; write: latch)
pub type TimerR = crate::FieldReader;
/// Field `TIMER` writer - Timer B, low 8 bits (read: counter; write: latch)
pub type TimerW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    /// Bits 0:7 - Timer B, low 8 bits (read: counter; write: latch)
    #[inline(always)]
    pub fn timer(&self) -> TimerR { TimerR::new(self.bits) }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TI2BLO").field("timer", &self.timer()).finish()
    }
}
impl W {
    /// Bits 0:7 - Timer B, low 8 bits (read: counter; write: latch)
    #[inline(always)]
    pub fn timer(&mut self) -> TimerW<'_, Ti2bloSpec> { TimerW::new(self, 0) }
}
/// Timer B bits 7:0 (as TALO)
///
/// You can [`read`](crate::Reg::read) this register and get [`ti2blo::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ti2blo::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct Ti2bloSpec;
impl crate::RegisterSpec for Ti2bloSpec {
    type Ux = u8;
}
/// `read()` method returns [`ti2blo::R`](R) reader structure
impl crate::Readable for Ti2bloSpec {}
/// `write(|w| ..)` method takes [`ti2blo::W`](W) writer structure
impl crate::Writable for Ti2bloSpec {
    type Safety = crate::Unsafe;
}
/// `reset()` method sets TI2BLO to value 0
impl crate::Resettable for Ti2bloSpec {}
