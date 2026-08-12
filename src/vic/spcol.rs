/// Register `SP%sCOL` reader
pub type R = crate::R<SpcolSpec>;
/// Register `SP%sCOL` writer
pub type W = crate::W<SpcolSpec>;
/// Sprite color
pub use super::extcol::Color;
/// Field `COLOR` reader - Sprite color
pub use super::extcol::ColorR;
/// Field `COLOR` writer - Sprite color
pub use super::extcol::ColorW;
impl R {
    /// Bits 0:3 - Sprite color
    #[inline(always)]
    pub fn color(&self) -> ColorR { ColorR::new(self.bits & 0x0f) }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPCOL").field("color", &self.color()).finish()
    }
}
impl W {
    /// Bits 0:3 - Sprite color
    #[inline(always)]
    pub fn color(&mut self) -> ColorW<'_, SpcolSpec> { ColorW::new(self, 0) }
}
/// Sprite color (bits 3:0)
///
/// You can [`read`](crate::Reg::read) this register and get [`spcol::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spcol::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct SpcolSpec;
impl crate::RegisterSpec for SpcolSpec {
    type Ux = u8;
}
/// `read()` method returns [`spcol::R`](R) reader structure
impl crate::Readable for SpcolSpec {}
/// `write(|w| ..)` method takes [`spcol::W`](W) writer structure
impl crate::Writable for SpcolSpec {
    type Safety = crate::Unsafe;
}
/// `reset()` method sets SP%sCOL to value 0
impl crate::Resettable for SpcolSpec {}
