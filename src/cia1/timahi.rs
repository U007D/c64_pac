/// Register `TIMAHI` reader
pub type R = crate::R<TimahiSpec>;
/// Register `TIMAHI` writer
pub type W = crate::W<TimahiSpec>;
/// Field `TIMER` reader - Timer A, high 8 bits (read: counter; write: latch)
pub type TimerR = crate::FieldReader;
/// Field `TIMER` writer - Timer A, high 8 bits (read: counter; write: latch)
pub type TimerW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    /// Bits 0:7 - Timer A, high 8 bits (read: counter; write: latch)
    #[inline(always)]
    pub fn timer(&self) -> TimerR { TimerR::new(self.bits) }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIMAHI").field("timer", &self.timer()).finish()
    }
}
impl W {
    /// Bits 0:7 - Timer A, high 8 bits (read: counter; write: latch)
    #[inline(always)]
    pub fn timer(&mut self) -> TimerW<'_, TimahiSpec> { TimerW::new(self, 0) }
}
/// Timer A bits 15:8. Read: current count. Write: latch; if the timer is
/// stopped, writing also loads the counter.
///
/// You can [`read`](crate::Reg::read) this register and get [`timahi::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timahi::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct TimahiSpec;
impl crate::RegisterSpec for TimahiSpec {
    type Ux = u8;
}
/// `read()` method returns [`timahi::R`](R) reader structure
impl crate::Readable for TimahiSpec {}
/// `write(|w| ..)` method takes [`timahi::W`](W) writer structure
impl crate::Writable for TimahiSpec {
    type Safety = crate::Unsafe;
}
/// `reset()` method sets TIMAHI to value 0
impl crate::Resettable for TimahiSpec {}
