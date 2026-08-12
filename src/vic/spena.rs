/// Register `SPENA` reader
pub type R = crate::R<SpenaSpec>;
/// Register `SPENA` writer
pub type W = crate::W<SpenaSpec>;
/// Sprite %s enabled
pub use super::scroly::Enable;
/// Field `SPRITE(0-7)` reader - Sprite %s enabled
pub use super::scroly::ScreenR as SpriteR;
/// Field `SPRITE(0-7)` writer - Sprite %s enabled
pub use super::scroly::ScreenW as SpriteW;
impl R {
    /// Sprite (0-7) enabled
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
    /// Sprite (0-7) enabled
    #[inline(always)]
    pub fn sprite_iter(&self) -> impl Iterator<Item = SpriteR> + '_ {
        (0..8).map(move |n| SpriteR::new(((self.bits >> n) & 1) != 0))
    }

    /// Bit 0 - Sprite 0 enabled
    #[inline(always)]
    pub fn sprite0(&self) -> SpriteR { SpriteR::new((self.bits & 1) != 0) }

    /// Bit 1 - Sprite 1 enabled
    #[inline(always)]
    pub fn sprite1(&self) -> SpriteR { SpriteR::new(((self.bits >> 1) & 1) != 0) }

    /// Bit 2 - Sprite 2 enabled
    #[inline(always)]
    pub fn sprite2(&self) -> SpriteR { SpriteR::new(((self.bits >> 2) & 1) != 0) }

    /// Bit 3 - Sprite 3 enabled
    #[inline(always)]
    pub fn sprite3(&self) -> SpriteR { SpriteR::new(((self.bits >> 3) & 1) != 0) }

    /// Bit 4 - Sprite 4 enabled
    #[inline(always)]
    pub fn sprite4(&self) -> SpriteR { SpriteR::new(((self.bits >> 4) & 1) != 0) }

    /// Bit 5 - Sprite 5 enabled
    #[inline(always)]
    pub fn sprite5(&self) -> SpriteR { SpriteR::new(((self.bits >> 5) & 1) != 0) }

    /// Bit 6 - Sprite 6 enabled
    #[inline(always)]
    pub fn sprite6(&self) -> SpriteR { SpriteR::new(((self.bits >> 6) & 1) != 0) }

    /// Bit 7 - Sprite 7 enabled
    #[inline(always)]
    pub fn sprite7(&self) -> SpriteR { SpriteR::new(((self.bits >> 7) & 1) != 0) }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPENA")
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
    /// Sprite (0-7) enabled
    ///
    /// <div class="warning">`n` is number of field in register. `n == 0`
    /// corresponds to `SPRITE0` field.</div>
    #[inline(always)]
    pub fn sprite(&mut self, n: u8) -> SpriteW<'_, SpenaSpec> {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        SpriteW::new(self, n)
    }

    /// Bit 0 - Sprite 0 enabled
    #[inline(always)]
    pub fn sprite0(&mut self) -> SpriteW<'_, SpenaSpec> { SpriteW::new(self, 0) }

    /// Bit 1 - Sprite 1 enabled
    #[inline(always)]
    pub fn sprite1(&mut self) -> SpriteW<'_, SpenaSpec> { SpriteW::new(self, 1) }

    /// Bit 2 - Sprite 2 enabled
    #[inline(always)]
    pub fn sprite2(&mut self) -> SpriteW<'_, SpenaSpec> { SpriteW::new(self, 2) }

    /// Bit 3 - Sprite 3 enabled
    #[inline(always)]
    pub fn sprite3(&mut self) -> SpriteW<'_, SpenaSpec> { SpriteW::new(self, 3) }

    /// Bit 4 - Sprite 4 enabled
    #[inline(always)]
    pub fn sprite4(&mut self) -> SpriteW<'_, SpenaSpec> { SpriteW::new(self, 4) }

    /// Bit 5 - Sprite 5 enabled
    #[inline(always)]
    pub fn sprite5(&mut self) -> SpriteW<'_, SpenaSpec> { SpriteW::new(self, 5) }

    /// Bit 6 - Sprite 6 enabled
    #[inline(always)]
    pub fn sprite6(&mut self) -> SpriteW<'_, SpenaSpec> { SpriteW::new(self, 6) }

    /// Bit 7 - Sprite 7 enabled
    #[inline(always)]
    pub fn sprite7(&mut self) -> SpriteW<'_, SpenaSpec> { SpriteW::new(self, 7) }
}
/// Sprite enable, bit n = sprite n
///
/// You can [`read`](crate::Reg::read) this register and get [`spena::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spena::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct SpenaSpec;
impl crate::RegisterSpec for SpenaSpec {
    type Ux = u8;
}
/// `read()` method returns [`spena::R`](R) reader structure
impl crate::Readable for SpenaSpec {}
/// `write(|w| ..)` method takes [`spena::W`](W) writer structure
impl crate::Writable for SpenaSpec {
    type Safety = crate::Unsafe;
}
/// `reset()` method sets SPENA to value 0
impl crate::Resettable for SpenaSpec {}
