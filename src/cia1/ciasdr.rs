/// Register `CIASDR` reader
pub type R = crate::R<CiasdrSpec>;
/// Register `CIASDR` writer
pub type W = crate::W<CiasdrSpec>;
/// Field `DATA` reader - Serial shift-register byte
pub type DataR = crate::FieldReader;
/// Field `DATA` writer - Serial shift-register byte
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    /// Bits 0:7 - Serial shift-register byte
    #[inline(always)]
    pub fn data(&self) -> DataR { DataR::new(self.bits) }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CIASDR").field("data", &self.data()).finish()
    }
}
impl W {
    /// Bits 0:7 - Serial shift-register byte
    #[inline(always)]
    pub fn data(&mut self) -> DataW<'_, CiasdrSpec> { DataW::new(self, 0) }
}
/// Serial shift register on SP pin, clocked by CNT; direction set by
/// CRA.SPMODE. Output rate is timer A underflow / 2.
///
/// You can [`read`](crate::Reg::read) this register and get [`ciasdr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ciasdr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct CiasdrSpec;
impl crate::RegisterSpec for CiasdrSpec {
    type Ux = u8;
}
/// `read()` method returns [`ciasdr::R`](R) reader structure
impl crate::Readable for CiasdrSpec {}
/// `write(|w| ..)` method takes [`ciasdr::W`](W) writer structure
impl crate::Writable for CiasdrSpec {
    type Safety = crate::Unsafe;
}
/// `reset()` method sets CIASDR to value 0
impl crate::Resettable for CiasdrSpec {}
