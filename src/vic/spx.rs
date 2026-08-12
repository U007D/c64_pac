/// Register `SP%sX` reader
pub type R = crate::R<SpxSpec>;
/// Register `SP%sX` writer
pub type W = crate::W<SpxSpec>;
/// Field `POSITION` reader - Sprite X position, low 8 bits (bit 8 in MSIGX)
pub type PositionR = crate::FieldReader;
/// Field `POSITION` writer - Sprite X position, low 8 bits (bit 8 in MSIGX)
pub type PositionW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    /// Bits 0:7 - Sprite X position, low 8 bits (bit 8 in MSIGX)
    #[inline(always)]
    pub fn position(&self) -> PositionR { PositionR::new(self.bits) }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPX").field("position", &self.position()).finish()
    }
}
impl W {
    /// Bits 0:7 - Sprite X position, low 8 bits (bit 8 in MSIGX)
    #[inline(always)]
    pub fn position(&mut self) -> PositionW<'_, SpxSpec> { PositionW::new(self, 0) }
}
/// Sprite X position bits 7:0 (bit 8 in MSIGX)
///
/// You can [`read`](crate::Reg::read) this register and get [`spx::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spx::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct SpxSpec;
impl crate::RegisterSpec for SpxSpec {
    type Ux = u8;
}
/// `read()` method returns [`spx::R`](R) reader structure
impl crate::Readable for SpxSpec {}
/// `write(|w| ..)` method takes [`spx::W`](W) writer structure
impl crate::Writable for SpxSpec {
    type Safety = crate::Unsafe;
}
/// `reset()` method sets SP%sX to value 0
impl crate::Resettable for SpxSpec {}
