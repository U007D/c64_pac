/// Register `VICIRQ_W` writer
pub type W = crate::W<VicirqWSpec>;
/// Acknowledge raster-compare interrupt
///
/// Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ack {
    /// 0: `0`
    Keep = 0,
    /// 1: `1`
    Ack = 1,
}
impl From<Ack> for bool {
    #[inline(always)]
    fn from(variant: Ack) -> Self { variant as u8 != 0 }
}
/// Field `RASTER` writer - Acknowledge raster-compare interrupt
pub type RasterW<'a, REG> = crate::BitWriter1C<'a, REG, Ack>;
impl<'a, REG> RasterW<'a, REG> where REG: crate::Writable + crate::RegisterSpec, {
    /// `0`
    #[inline(always)]
    pub fn keep(self) -> &'a mut crate::W<REG> { self.variant(Ack::Keep) }

    /// `1`
    #[inline(always)]
    pub fn ack(self) -> &'a mut crate::W<REG> { self.variant(Ack::Ack) }
}
/// Field `SPRITE_BG_COLLISION` writer - Acknowledge sprite-background collision
/// interrupt
pub use RasterW as SpriteBgCollisionW;
/// Field `SPRITE_SPRITE_COLLISION` writer - Acknowledge sprite-sprite collision
/// interrupt
pub use RasterW as SpriteSpriteCollisionW;
/// Field `LIGHT_PEN` writer - Acknowledge light-pen interrupt
pub use RasterW as LightPenW;
impl W {
    /// Bit 0 - Acknowledge raster-compare interrupt
    #[inline(always)]
    pub fn raster(&mut self) -> RasterW<'_, VicirqWSpec> { RasterW::new(self, 0) }

    /// Bit 1 - Acknowledge sprite-background collision interrupt
    #[inline(always)]
    pub fn sprite_bg_collision(&mut self) -> SpriteBgCollisionW<'_, VicirqWSpec> {
        SpriteBgCollisionW::new(self, 1)
    }

    /// Bit 2 - Acknowledge sprite-sprite collision interrupt
    #[inline(always)]
    pub fn sprite_sprite_collision(&mut self) -> SpriteSpriteCollisionW<'_, VicirqWSpec> {
        SpriteSpriteCollisionW::new(self, 2)
    }

    /// Bit 3 - Acknowledge light-pen interrupt
    #[inline(always)]
    pub fn light_pen(&mut self) -> LightPenW<'_, VicirqWSpec> { LightPenW::new(self, 3) }
}
/// Acknowledge latched VIC interrupts (write-only alternate view of 0xD019,
/// write-1-to-clear). Writing Ack to a bit clears that source; Keep (0) leaves
/// its flag pending. There is no read here - read status via VICIRQ_R.
/// SIDE-EFFECT: an enabled source left un-acked re-asserts the IRQ immediately.
///
/// You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vicirq_w::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct VicirqWSpec;
impl crate::RegisterSpec for VicirqWSpec {
    type Ux = u8;
}
/// `write(|w| ..)` method takes [`vicirq_w::W`](W) writer structure
impl crate::Writable for VicirqWSpec {
    type Safety = crate::Unsafe;

    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0x0f;
}
/// `reset()` method sets VICIRQ_W to value 0
impl crate::Resettable for VicirqWSpec {}
