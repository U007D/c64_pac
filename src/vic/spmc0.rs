/// Register `SPMC0` reader
pub type R = crate::R<Spmc0Spec>;
/// Register `SPMC0` writer
pub type W = crate::W<Spmc0Spec>;
/// Shared sprite multicolor 0
pub use super::extcol::Color;
/// Field `COLOR` reader - Shared sprite multicolor 0
pub use super::extcol::ColorR;
/// Field `COLOR` writer - Shared sprite multicolor 0
pub use super::extcol::ColorW;
impl R {
    /// Bits 0:3 - Shared sprite multicolor 0
    #[inline(always)]
    pub fn color(&self) -> ColorR { ColorR::new(self.bits & 0x0f) }
}
impl W {
    /// Bits 0:3 - Shared sprite multicolor 0
    #[inline(always)]
    pub fn color(&mut self) -> ColorW<'_, Spmc0Spec> { ColorW::new(self, 0) }
}
/// Sprite multicolor 0 (bits 3:0)
///
/// You can [`read`](crate::Reg::read) this register and get [`spmc0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spmc0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct Spmc0Spec;
impl crate::RegisterSpec for Spmc0Spec {
    type Ux = u8;
}
/// `read()` method returns [`spmc0::R`](R) reader structure
impl crate::Readable for Spmc0Spec {}
/// `write(|w| ..)` method takes [`spmc0::W`](W) writer structure
impl crate::Writable for Spmc0Spec {
    type Safety = crate::Unsafe;
}
/// `reset()` method sets SPMC0 to value 0
impl crate::Resettable for Spmc0Spec {}
