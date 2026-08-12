/// Register `TIMBHI` reader
pub type R = crate::R<TimbhiSpec>;
/// Register `TIMBHI` writer
pub type W = crate::W<TimbhiSpec>;
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
    pub fn timer(&mut self) -> TimerW<'_, TimbhiSpec> { TimerW::new(self, 0) }
}
/// Timer B bits 15:8 (as TAHI)
///
/// You can [`read`](crate::Reg::read) this register and get [`timbhi::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timbhi::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct TimbhiSpec;
impl crate::RegisterSpec for TimbhiSpec {
    type Ux = u8;
}
/// `read()` method returns [`timbhi::R`](R) reader structure
impl crate::Readable for TimbhiSpec {}
/// `write(|w| ..)` method takes [`timbhi::W`](W) writer structure
impl crate::Writable for TimbhiSpec {
    type Safety = crate::Unsafe;
}
/// `reset()` method sets TIMBHI to value 0
impl crate::Resettable for TimbhiSpec {}
