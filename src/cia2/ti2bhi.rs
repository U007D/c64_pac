/// Register `TI2BHI` reader
pub type R = crate::R<Ti2bhiSpec>;
/// Register `TI2BHI` writer
pub type W = crate::W<Ti2bhiSpec>;
/// Field `TIMER` reader - Timer B, high 8 bits (read: counter; write: latch)
pub type TimerR = crate::FieldReader;
/// Field `TIMER` writer - Timer B, high 8 bits (read: counter; write: latch)
pub type TimerW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    /// Bits 0:7 - Timer B, high 8 bits (read: counter; write: latch)
    #[inline(always)]
    pub fn timer(&self) -> TimerR { TimerR::new(self.bits) }
}
impl W {
    /// Bits 0:7 - Timer B, high 8 bits (read: counter; write: latch)
    #[inline(always)]
    pub fn timer(&mut self) -> TimerW<'_, Ti2bhiSpec> { TimerW::new(self, 0) }
}
/// Timer B bits 15:8 (as TAHI)
///
/// You can [`read`](crate::Reg::read) this register and get [`ti2bhi::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ti2bhi::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct Ti2bhiSpec;
impl crate::RegisterSpec for Ti2bhiSpec {
    type Ux = u8;
}
/// `read()` method returns [`ti2bhi::R`](R) reader structure
impl crate::Readable for Ti2bhiSpec {}
/// `write(|w| ..)` method takes [`ti2bhi::W`](W) writer structure
impl crate::Writable for Ti2bhiSpec {
    type Safety = crate::Unsafe;
}
/// `reset()` method sets TI2BHI to value 0
impl crate::Resettable for Ti2bhiSpec {}
