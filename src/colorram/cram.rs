/// Register `CRAM%s` reader
pub type R = crate::R<CramSpec>;
/// Register `CRAM%s` writer
pub type W = crate::W<CramSpec>;
/// Color of this cell
pub use crate::vic::extcol::Color;
/// Field `COLOR` reader - Color of this cell
pub use crate::vic::extcol::ColorR;
/// Field `COLOR` writer - Color of this cell
pub use crate::vic::extcol::ColorW;
impl R {
    /// Bits 0:3 - Color of this cell
    #[inline(always)]
    pub fn color(&self) -> ColorR { ColorR::new(self.bits & 0x0f) }
}
impl W {
    /// Bits 0:3 - Color of this cell
    #[inline(always)]
    pub fn color(&mut self) -> ColorW<'_, CramSpec> { ColorW::new(self, 0) }
}
/// Color cell (bits 3:0)
///
/// You can [`read`](crate::Reg::read) this register and get [`cram::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cram::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct CramSpec;
impl crate::RegisterSpec for CramSpec {
    type Ux = u8;
}
/// `read()` method returns [`cram::R`](R) reader structure
impl crate::Readable for CramSpec {}
/// `write(|w| ..)` method takes [`cram::W`](W) writer structure
impl crate::Writable for CramSpec {
    type Safety = crate::Unsafe;
}
/// `reset()` method sets CRAM%s to value 0
impl crate::Resettable for CramSpec {}
