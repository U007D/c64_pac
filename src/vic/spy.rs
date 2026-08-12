/// Register `SP%sY` reader
pub type R = crate::R<SpySpec>;
/// Register `SP%sY` writer
pub type W = crate::W<SpySpec>;
/// Field `POSITION` reader - Sprite Y position
pub type PositionR = crate::FieldReader;
/// Field `POSITION` writer - Sprite Y position
pub type PositionW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    /// Bits 0:7 - Sprite Y position
    #[inline(always)]
    pub fn position(&self) -> PositionR { PositionR::new(self.bits) }
}
impl W {
    /// Bits 0:7 - Sprite Y position
    #[inline(always)]
    pub fn position(&mut self) -> PositionW<'_, SpySpec> { PositionW::new(self, 0) }
}
/// Sprite Y position
///
/// You can [`read`](crate::Reg::read) this register and get [`spy::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spy::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct SpySpec;
impl crate::RegisterSpec for SpySpec {
    type Ux = u8;
}
/// `read()` method returns [`spy::R`](R) reader structure
impl crate::Readable for SpySpec {}
/// `write(|w| ..)` method takes [`spy::W`](W) writer structure
impl crate::Writable for SpySpec {
    type Safety = crate::Unsafe;
}
/// `reset()` method sets SP%sY to value 0
impl crate::Resettable for SpySpec {}
