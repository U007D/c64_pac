/// Register `CI2SDR` reader
pub type R = crate::R<Ci2sdrSpec>;
/// Register `CI2SDR` writer
pub type W = crate::W<Ci2sdrSpec>;
/// Field `DATA` reader - Serial shift-register byte
pub type DataR = crate::FieldReader;
/// Field `DATA` writer - Serial shift-register byte
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    /// Bits 0:7 - Serial shift-register byte
    #[inline(always)]
    pub fn data(&self) -> DataR { DataR::new(self.bits) }
}
impl W {
    /// Bits 0:7 - Serial shift-register byte
    #[inline(always)]
    pub fn data(&mut self) -> DataW<'_, Ci2sdrSpec> { DataW::new(self, 0) }
}
/// Serial shift register on SP2, clocked by CNT2; direction set by CRA.SPMODE
///
/// You can [`read`](crate::Reg::read) this register and get [`ci2sdr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ci2sdr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct Ci2sdrSpec;
impl crate::RegisterSpec for Ci2sdrSpec {
    type Ux = u8;
}
/// `read()` method returns [`ci2sdr::R`](R) reader structure
impl crate::Readable for Ci2sdrSpec {}
/// `write(|w| ..)` method takes [`ci2sdr::W`](W) writer structure
impl crate::Writable for Ci2sdrSpec {
    type Safety = crate::Unsafe;
}
/// `reset()` method sets CI2SDR to value 0
impl crate::Resettable for Ci2sdrSpec {}
