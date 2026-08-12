/// Register `IRQMASK` reader
pub type R = crate::R<IrqmaskSpec>;
/// Register `IRQMASK` writer
pub type W = crate::W<IrqmaskSpec>;
/// Enable raster interrupt
pub use super::scroly::Enable;
/// Field `RASTER` reader - Enable raster interrupt
pub use super::scroly::ScreenR as RasterR;
/// Field `SPRITE_BG_COLLISION` reader - Enable sprite-background collision
/// interrupt
pub use super::scroly::ScreenR as SpriteBgCollisionR;
/// Field `SPRITE_SPRITE_COLLISION` reader - Enable sprite-sprite collision
/// interrupt
pub use super::scroly::ScreenR as SpriteSpriteCollisionR;
/// Field `LIGHT_PEN` reader - Enable light pen interrupt
pub use super::scroly::ScreenR as LightPenR;
/// Field `RASTER` writer - Enable raster interrupt
pub use super::scroly::ScreenW as RasterW;
/// Field `SPRITE_BG_COLLISION` writer - Enable sprite-background collision
/// interrupt
pub use super::scroly::ScreenW as SpriteBgCollisionW;
/// Field `SPRITE_SPRITE_COLLISION` writer - Enable sprite-sprite collision
/// interrupt
pub use super::scroly::ScreenW as SpriteSpriteCollisionW;
/// Field `LIGHT_PEN` writer - Enable light pen interrupt
pub use super::scroly::ScreenW as LightPenW;
impl R {
    /// Bit 0 - Enable raster interrupt
    #[inline(always)]
    pub fn raster(&self) -> RasterR { RasterR::new((self.bits & 1) != 0) }

    /// Bit 1 - Enable sprite-background collision interrupt
    #[inline(always)]
    pub fn sprite_bg_collision(&self) -> SpriteBgCollisionR {
        SpriteBgCollisionR::new(((self.bits >> 1) & 1) != 0)
    }

    /// Bit 2 - Enable sprite-sprite collision interrupt
    #[inline(always)]
    pub fn sprite_sprite_collision(&self) -> SpriteSpriteCollisionR {
        SpriteSpriteCollisionR::new(((self.bits >> 2) & 1) != 0)
    }

    /// Bit 3 - Enable light pen interrupt
    #[inline(always)]
    pub fn light_pen(&self) -> LightPenR { LightPenR::new(((self.bits >> 3) & 1) != 0) }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IRQMASK")
         .field("raster", &self.raster())
         .field("sprite_bg_collision", &self.sprite_bg_collision())
         .field("sprite_sprite_collision", &self.sprite_sprite_collision())
         .field("light_pen", &self.light_pen())
         .finish()
    }
}
impl W {
    /// Bit 0 - Enable raster interrupt
    #[inline(always)]
    pub fn raster(&mut self) -> RasterW<'_, IrqmaskSpec> { RasterW::new(self, 0) }

    /// Bit 1 - Enable sprite-background collision interrupt
    #[inline(always)]
    pub fn sprite_bg_collision(&mut self) -> SpriteBgCollisionW<'_, IrqmaskSpec> {
        SpriteBgCollisionW::new(self, 1)
    }

    /// Bit 2 - Enable sprite-sprite collision interrupt
    #[inline(always)]
    pub fn sprite_sprite_collision(&mut self) -> SpriteSpriteCollisionW<'_, IrqmaskSpec> {
        SpriteSpriteCollisionW::new(self, 2)
    }

    /// Bit 3 - Enable light pen interrupt
    #[inline(always)]
    pub fn light_pen(&mut self) -> LightPenW<'_, IrqmaskSpec> { LightPenW::new(self, 3) }
}
/// Interrupt enable for the four VICIRQ_R sources (same bit positions)
///
/// You can [`read`](crate::Reg::read) this register and get [`irqmask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irqmask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct IrqmaskSpec;
impl crate::RegisterSpec for IrqmaskSpec {
    type Ux = u8;
}
/// `read()` method returns [`irqmask::R`](R) reader structure
impl crate::Readable for IrqmaskSpec {}
/// `write(|w| ..)` method takes [`irqmask::W`](W) writer structure
impl crate::Writable for IrqmaskSpec {
    type Safety = crate::Unsafe;
}
/// `reset()` method sets IRQMASK to value 0
impl crate::Resettable for IrqmaskSpec {}
