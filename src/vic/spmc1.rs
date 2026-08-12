/// Register `SPMC1` reader
pub type R = crate::R<Spmc1Spec>;
/// Register `SPMC1` writer
pub type W = crate::W<Spmc1Spec>;
/// Shared sprite multicolor 1
pub use super::extcol::Color;
/// Field `COLOR` reader - Shared sprite multicolor 1
pub use super::extcol::ColorR;
/// Field `COLOR` writer - Shared sprite multicolor 1
pub use super::extcol::ColorW;
impl R {
    /// Bits 0:3 - Shared sprite multicolor 1
    #[inline(always)]
    pub fn color(&self) -> ColorR { ColorR::new(self.bits & 0x0f) }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPMC1").field("color", &self.color()).finish()
    }
}
impl W {
    /// Bits 0:3 - Shared sprite multicolor 1
    #[inline(always)]
    pub fn color(&mut self) -> ColorW<'_, Spmc1Spec> { ColorW::new(self, 0) }
}
/// Sprite multicolor 1 (bits 3:0)
///
/// You can [`read`](crate::Reg::read) this register and get [`spmc1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spmc1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct Spmc1Spec;
impl crate::RegisterSpec for Spmc1Spec {
    type Ux = u8;
}
/// `read()` method returns [`spmc1::R`](R) reader structure
impl crate::Readable for Spmc1Spec {}
/// `write(|w| ..)` method takes [`spmc1::W`](W) writer structure
impl crate::Writable for Spmc1Spec {
    type Safety = crate::Unsafe;
}
/// `reset()` method sets SPMC1 to value 0
impl crate::Resettable for Spmc1Spec {}
