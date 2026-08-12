/// Register `YXPAND` reader
pub type R = crate::R<YxpandSpec>;
/// Register `YXPAND` writer
pub type W = crate::W<YxpandSpec>;
/// Sprite %s vertically expanded
pub use super::scroly::Enable;
/// Field `SPRITE(0-7)` reader - Sprite %s vertically expanded
pub use super::scroly::ScreenR as SpriteR;
/// Field `SPRITE(0-7)` writer - Sprite %s vertically expanded
pub use super::scroly::ScreenW as SpriteW;
impl R {
    /// Sprite (0-7) vertically expanded
    ///
    /// <div class="warning">`n` is number of field in register. `n == 0`
    /// corresponds to `SPRITE0` field.</div>
    #[inline(always)]
    pub fn sprite(&self, n: u8) -> SpriteR {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        SpriteR::new(((self.bits >> n) & 1) != 0)
    }

    /// Iterator for array of:
    /// Sprite (0-7) vertically expanded
    #[inline(always)]
    pub fn sprite_iter(&self) -> impl Iterator<Item = SpriteR> + '_ {
        (0..8).map(move |n| SpriteR::new(((self.bits >> n) & 1) != 0))
    }

    /// Bit 0 - Sprite 0 vertically expanded
    #[inline(always)]
    pub fn sprite0(&self) -> SpriteR { SpriteR::new((self.bits & 1) != 0) }

    /// Bit 1 - Sprite 1 vertically expanded
    #[inline(always)]
    pub fn sprite1(&self) -> SpriteR { SpriteR::new(((self.bits >> 1) & 1) != 0) }

    /// Bit 2 - Sprite 2 vertically expanded
    #[inline(always)]
    pub fn sprite2(&self) -> SpriteR { SpriteR::new(((self.bits >> 2) & 1) != 0) }

    /// Bit 3 - Sprite 3 vertically expanded
    #[inline(always)]
    pub fn sprite3(&self) -> SpriteR { SpriteR::new(((self.bits >> 3) & 1) != 0) }

    /// Bit 4 - Sprite 4 vertically expanded
    #[inline(always)]
    pub fn sprite4(&self) -> SpriteR { SpriteR::new(((self.bits >> 4) & 1) != 0) }

    /// Bit 5 - Sprite 5 vertically expanded
    #[inline(always)]
    pub fn sprite5(&self) -> SpriteR { SpriteR::new(((self.bits >> 5) & 1) != 0) }

    /// Bit 6 - Sprite 6 vertically expanded
    #[inline(always)]
    pub fn sprite6(&self) -> SpriteR { SpriteR::new(((self.bits >> 6) & 1) != 0) }

    /// Bit 7 - Sprite 7 vertically expanded
    #[inline(always)]
    pub fn sprite7(&self) -> SpriteR { SpriteR::new(((self.bits >> 7) & 1) != 0) }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("YXPAND")
         .field("sprite0", &self.sprite0())
         .field("sprite1", &self.sprite1())
         .field("sprite2", &self.sprite2())
         .field("sprite3", &self.sprite3())
         .field("sprite4", &self.sprite4())
         .field("sprite5", &self.sprite5())
         .field("sprite6", &self.sprite6())
         .field("sprite7", &self.sprite7())
         .finish()
    }
}
impl W {
    /// Sprite (0-7) vertically expanded
    ///
    /// <div class="warning">`n` is number of field in register. `n == 0`
    /// corresponds to `SPRITE0` field.</div>
    #[inline(always)]
    pub fn sprite(&mut self, n: u8) -> SpriteW<'_, YxpandSpec> {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        SpriteW::new(self, n)
    }

    /// Bit 0 - Sprite 0 vertically expanded
    #[inline(always)]
    pub fn sprite0(&mut self) -> SpriteW<'_, YxpandSpec> { SpriteW::new(self, 0) }

    /// Bit 1 - Sprite 1 vertically expanded
    #[inline(always)]
    pub fn sprite1(&mut self) -> SpriteW<'_, YxpandSpec> { SpriteW::new(self, 1) }

    /// Bit 2 - Sprite 2 vertically expanded
    #[inline(always)]
    pub fn sprite2(&mut self) -> SpriteW<'_, YxpandSpec> { SpriteW::new(self, 2) }

    /// Bit 3 - Sprite 3 vertically expanded
    #[inline(always)]
    pub fn sprite3(&mut self) -> SpriteW<'_, YxpandSpec> { SpriteW::new(self, 3) }

    /// Bit 4 - Sprite 4 vertically expanded
    #[inline(always)]
    pub fn sprite4(&mut self) -> SpriteW<'_, YxpandSpec> { SpriteW::new(self, 4) }

    /// Bit 5 - Sprite 5 vertically expanded
    #[inline(always)]
    pub fn sprite5(&mut self) -> SpriteW<'_, YxpandSpec> { SpriteW::new(self, 5) }

    /// Bit 6 - Sprite 6 vertically expanded
    #[inline(always)]
    pub fn sprite6(&mut self) -> SpriteW<'_, YxpandSpec> { SpriteW::new(self, 6) }

    /// Bit 7 - Sprite 7 vertically expanded
    #[inline(always)]
    pub fn sprite7(&mut self) -> SpriteW<'_, YxpandSpec> { SpriteW::new(self, 7) }
}
/// Sprite Y expansion, bit n = sprite n
///
/// You can [`read`](crate::Reg::read) this register and get [`yxpand::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`yxpand::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct YxpandSpec;
impl crate::RegisterSpec for YxpandSpec {
    type Ux = u8;
}
/// `read()` method returns [`yxpand::R`](R) reader structure
impl crate::Readable for YxpandSpec {}
/// `write(|w| ..)` method takes [`yxpand::W`](W) writer structure
impl crate::Writable for YxpandSpec {
    type Safety = crate::Unsafe;
}
/// `reset()` method sets YXPAND to value 0
impl crate::Resettable for YxpandSpec {}
