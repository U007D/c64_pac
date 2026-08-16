/// Register `TIMALO` reader
pub type R = crate::R<TimaloSpec>;
/// Register `TIMALO` writer
pub type W = crate::W<TimaloSpec>;
/// Field `TIMER` reader - Timer A, low 8 bits (read: counter; write: latch)
pub type TimerR = crate::FieldReader;
/// Field `TIMER` writer - Timer A, low 8 bits (read: counter; write: latch)
pub type TimerW<'a, REG> = crate::FieldWriter<'a, REG, 8, u8, crate::Safe>;
impl R {
    /// Bits 0:7 - Timer A, low 8 bits (read: counter; write: latch)
    #[inline(always)]
    pub fn timer(&self) -> TimerR { TimerR::new(self.bits) }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIMALO").field("timer", &self.timer()).finish()
    }
}
impl W {
    /// Bits 0:7 - Timer A, low 8 bits (read: counter; write: latch)
    #[inline(always)]
    pub fn timer(&mut self) -> TimerW<'_, TimaloSpec> { TimerW::new(self, 0) }
}
/// Timer A bits 7:0. Read: current count. Write: latch (reload value). HI/LO
/// reads are not latched together, so a 16-bit read can tear across a
/// decrement: read HI, LO, re-read HI and retry on change, or stop the timer.
///
/// You can [`read`](crate::Reg::read) this register and get [`timalo::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timalo::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct TimaloSpec;
impl crate::RegisterSpec for TimaloSpec {
    type Ux = u8;
}
/// `read()` method returns [`timalo::R`](R) reader structure
impl crate::Readable for TimaloSpec {}
/// `write(|w| ..)` method takes [`timalo::W`](W) writer structure
impl crate::Writable for TimaloSpec {
    type Safety = crate::Safe;
}
/// `reset()` method sets TIMALO to value 0
impl crate::Resettable for TimaloSpec {}
