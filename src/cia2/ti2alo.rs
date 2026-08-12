/// Register `TI2ALO` reader
pub type R = crate::R<Ti2aloSpec>;
/// Register `TI2ALO` writer
pub type W = crate::W<Ti2aloSpec>;
/// Field `TIMER` reader - Timer A, low 8 bits (read: counter; write: latch)
pub type TimerR = crate::FieldReader;
/// Field `TIMER` writer - Timer A, low 8 bits (read: counter; write: latch)
pub type TimerW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    /// Bits 0:7 - Timer A, low 8 bits (read: counter; write: latch)
    #[inline(always)]
    pub fn timer(&self) -> TimerR { TimerR::new(self.bits) }
}
impl W {
    /// Bits 0:7 - Timer A, low 8 bits (read: counter; write: latch)
    #[inline(always)]
    pub fn timer(&mut self) -> TimerW<'_, Ti2aloSpec> { TimerW::new(self, 0) }
}
/// Timer A bits 7:0. Read: current count. Write: latch (reload value). HI/LO
/// reads are not latched together, so a 16-bit read can tear across a
/// decrement: read HI, LO, re-read HI and retry on change, or stop the timer.
///
/// You can [`read`](crate::Reg::read) this register and get [`ti2alo::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ti2alo::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct Ti2aloSpec;
impl crate::RegisterSpec for Ti2aloSpec {
    type Ux = u8;
}
/// `read()` method returns [`ti2alo::R`](R) reader structure
impl crate::Readable for Ti2aloSpec {}
/// `write(|w| ..)` method takes [`ti2alo::W`](W) writer structure
impl crate::Writable for Ti2aloSpec {
    type Safety = crate::Unsafe;
}
/// `reset()` method sets TI2ALO to value 0
impl crate::Resettable for Ti2aloSpec {}
