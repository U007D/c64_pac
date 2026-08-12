/// Register `BGCOL%s` reader
pub type R = crate::R<BgcolSpec>;
/// Register `BGCOL%s` writer
pub type W = crate::W<BgcolSpec>;
/// Background color
pub use super::extcol::Color;
/// Field `COLOR` reader - Background color
pub use super::extcol::ColorR;
/// Field `COLOR` writer - Background color
pub use super::extcol::ColorW;
impl R {
    /// Bits 0:3 - Background color
    #[inline(always)]
    pub fn color(&self) -> ColorR { ColorR::new(self.bits & 0x0f) }
}
impl W {
    /// Bits 0:3 - Background color
    #[inline(always)]
    pub fn color(&mut self) -> ColorW<'_, BgcolSpec> { ColorW::new(self, 0) }
}
/// Background color (bits 3:0)
///
/// You can [`read`](crate::Reg::read) this register and get [`bgcol::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgcol::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct BgcolSpec;
impl crate::RegisterSpec for BgcolSpec {
    type Ux = u8;
}
/// `read()` method returns [`bgcol::R`](R) reader structure
impl crate::Readable for BgcolSpec {}
/// `write(|w| ..)` method takes [`bgcol::W`](W) writer structure
impl crate::Writable for BgcolSpec {
    type Safety = crate::Unsafe;
}
/// `reset()` method sets BGCOL%s to value 0
impl crate::Resettable for BgcolSpec {}
