/// Register `TI2AHI` reader
pub type R = crate::R<Ti2ahiSpec>;
/// Register `TI2AHI` writer
pub type W = crate::W<Ti2ahiSpec>;
/// Field `TIMER` reader - Timer A, high 8 bits (read: counter; write: latch)
pub type TimerR = crate::FieldReader;
/// Field `TIMER` writer - Timer A, high 8 bits (read: counter; write: latch)
pub type TimerW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    /// Bits 0:7 - Timer A, high 8 bits (read: counter; write: latch)
    #[inline(always)]
    pub fn timer(&self) -> TimerR { TimerR::new(self.bits) }
}
impl W {
    /// Bits 0:7 - Timer A, high 8 bits (read: counter; write: latch)
    #[inline(always)]
    pub fn timer(&mut self) -> TimerW<'_, Ti2ahiSpec> { TimerW::new(self, 0) }
}
/// Timer A bits 15:8. Read: current count. Write: latch; if the timer is
/// stopped, writing also loads the counter.
///
/// You can [`read`](crate::Reg::read) this register and get [`ti2ahi::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ti2ahi::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct Ti2ahiSpec;
impl crate::RegisterSpec for Ti2ahiSpec {
    type Ux = u8;
}
/// `read()` method returns [`ti2ahi::R`](R) reader structure
impl crate::Readable for Ti2ahiSpec {}
/// `write(|w| ..)` method takes [`ti2ahi::W`](W) writer structure
impl crate::Writable for Ti2ahiSpec {
    type Safety = crate::Unsafe;
}
/// `reset()` method sets TI2AHI to value 0
impl crate::Resettable for Ti2ahiSpec {}
