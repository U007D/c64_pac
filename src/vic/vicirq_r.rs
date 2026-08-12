/// Register `VICIRQ_R` reader
pub type R = crate::R<VicirqRSpec>;
/// Raster compare reached
///
/// Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Latch {
    /// 0: `0`
    Inactive = 0,
    /// 1: `1`
    Latched = 1,
}
impl From<Latch> for bool {
    #[inline(always)]
    fn from(variant: Latch) -> Self { variant as u8 != 0 }
}
/// Field `RASTER` reader - Raster compare reached
pub type RasterR = crate::BitReader<Latch>;
impl RasterR {
    /// Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Latch {
        match self.bits {
            false => Latch::Inactive,
            true => Latch::Latched,
        }
    }

    /// `0`
    #[inline(always)]
    pub fn is_inactive(&self) -> bool { *self == Latch::Inactive }

    /// `1`
    #[inline(always)]
    pub fn is_latched(&self) -> bool { *self == Latch::Latched }
}
/// Field `SPRITE_BG_COLLISION` reader - Sprite-background collision
pub use RasterR as SpriteBgCollisionR;
/// Field `SPRITE_SPRITE_COLLISION` reader - Sprite-sprite collision
pub use RasterR as SpriteSpriteCollisionR;
/// Field `LIGHT_PEN` reader - Light pen
pub use RasterR as LightPenR;
/// Field `IRQ` reader - Any enabled source latched
pub use RasterR as IrqR;
impl R {
    /// Bit 0 - Raster compare reached
    #[inline(always)]
    pub fn raster(&self) -> RasterR { RasterR::new((self.bits & 1) != 0) }

    /// Bit 1 - Sprite-background collision
    #[inline(always)]
    pub fn sprite_bg_collision(&self) -> SpriteBgCollisionR {
        SpriteBgCollisionR::new(((self.bits >> 1) & 1) != 0)
    }

    /// Bit 2 - Sprite-sprite collision
    #[inline(always)]
    pub fn sprite_sprite_collision(&self) -> SpriteSpriteCollisionR {
        SpriteSpriteCollisionR::new(((self.bits >> 2) & 1) != 0)
    }

    /// Bit 3 - Light pen
    #[inline(always)]
    pub fn light_pen(&self) -> LightPenR { LightPenR::new(((self.bits >> 3) & 1) != 0) }

    /// Bit 7 - Any enabled source latched
    #[inline(always)]
    pub fn irq(&self) -> IrqR { IrqR::new(((self.bits >> 7) & 1) != 0) }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VICIRQ_R")
         .field("raster", &self.raster())
         .field("sprite_bg_collision", &self.sprite_bg_collision())
         .field("sprite_sprite_collision", &self.sprite_sprite_collision())
         .field("light_pen", &self.light_pen())
         .field("irq", &self.irq())
         .finish()
    }
}
/// Interrupt latch (read-only view of 0xD019): which VIC sources have fired.
/// Bit 7 reads Latched while any enabled source is latched; bits 4-6 read as 1.
/// Acknowledge via VICIRQ_W.
///
/// You can [`read`](crate::Reg::read) this register and get [`vicirq_r::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct VicirqRSpec;
impl crate::RegisterSpec for VicirqRSpec {
    type Ux = u8;
}
/// `read()` method returns [`vicirq_r::R`](R) reader structure
impl crate::Readable for VicirqRSpec {}
/// `reset()` method sets VICIRQ_R to value 0
impl crate::Resettable for VicirqRSpec {}
