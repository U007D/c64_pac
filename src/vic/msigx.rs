/// Register `MSIGX` reader
pub type R = crate::R<MsigxSpec>;
/// Register `MSIGX` writer
pub type W = crate::W<MsigxSpec>;
/// Field `SPRITE(0-7)` reader - Sprite %s X position bit 8
pub type SpriteR = crate::BitReader;
/// Field `SPRITE(0-7)` writer - Sprite %s X position bit 8
pub type SpriteW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    /// Sprite (0-7) X position bit 8
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
    /// Sprite (0-7) X position bit 8
    #[inline(always)]
    pub fn sprite_iter(&self) -> impl Iterator<Item = SpriteR> + '_ {
        (0..8).map(move |n| SpriteR::new(((self.bits >> n) & 1) != 0))
    }

    /// Bit 0 - Sprite 0 X position bit 8
    #[inline(always)]
    pub fn sprite0(&self) -> SpriteR { SpriteR::new((self.bits & 1) != 0) }

    /// Bit 1 - Sprite 1 X position bit 8
    #[inline(always)]
    pub fn sprite1(&self) -> SpriteR { SpriteR::new(((self.bits >> 1) & 1) != 0) }

    /// Bit 2 - Sprite 2 X position bit 8
    #[inline(always)]
    pub fn sprite2(&self) -> SpriteR { SpriteR::new(((self.bits >> 2) & 1) != 0) }

    /// Bit 3 - Sprite 3 X position bit 8
    #[inline(always)]
    pub fn sprite3(&self) -> SpriteR { SpriteR::new(((self.bits >> 3) & 1) != 0) }

    /// Bit 4 - Sprite 4 X position bit 8
    #[inline(always)]
    pub fn sprite4(&self) -> SpriteR { SpriteR::new(((self.bits >> 4) & 1) != 0) }

    /// Bit 5 - Sprite 5 X position bit 8
    #[inline(always)]
    pub fn sprite5(&self) -> SpriteR { SpriteR::new(((self.bits >> 5) & 1) != 0) }

    /// Bit 6 - Sprite 6 X position bit 8
    #[inline(always)]
    pub fn sprite6(&self) -> SpriteR { SpriteR::new(((self.bits >> 6) & 1) != 0) }

    /// Bit 7 - Sprite 7 X position bit 8
    #[inline(always)]
    pub fn sprite7(&self) -> SpriteR { SpriteR::new(((self.bits >> 7) & 1) != 0) }
}
impl W {
    /// Sprite (0-7) X position bit 8
    ///
    /// <div class="warning">`n` is number of field in register. `n == 0`
    /// corresponds to `SPRITE0` field.</div>
    #[inline(always)]
    pub fn sprite(&mut self, n: u8) -> SpriteW<'_, MsigxSpec> {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        SpriteW::new(self, n)
    }

    /// Bit 0 - Sprite 0 X position bit 8
    #[inline(always)]
    pub fn sprite0(&mut self) -> SpriteW<'_, MsigxSpec> { SpriteW::new(self, 0) }

    /// Bit 1 - Sprite 1 X position bit 8
    #[inline(always)]
    pub fn sprite1(&mut self) -> SpriteW<'_, MsigxSpec> { SpriteW::new(self, 1) }

    /// Bit 2 - Sprite 2 X position bit 8
    #[inline(always)]
    pub fn sprite2(&mut self) -> SpriteW<'_, MsigxSpec> { SpriteW::new(self, 2) }

    /// Bit 3 - Sprite 3 X position bit 8
    #[inline(always)]
    pub fn sprite3(&mut self) -> SpriteW<'_, MsigxSpec> { SpriteW::new(self, 3) }

    /// Bit 4 - Sprite 4 X position bit 8
    #[inline(always)]
    pub fn sprite4(&mut self) -> SpriteW<'_, MsigxSpec> { SpriteW::new(self, 4) }

    /// Bit 5 - Sprite 5 X position bit 8
    #[inline(always)]
    pub fn sprite5(&mut self) -> SpriteW<'_, MsigxSpec> { SpriteW::new(self, 5) }

    /// Bit 6 - Sprite 6 X position bit 8
    #[inline(always)]
    pub fn sprite6(&mut self) -> SpriteW<'_, MsigxSpec> { SpriteW::new(self, 6) }

    /// Bit 7 - Sprite 7 X position bit 8
    #[inline(always)]
    pub fn sprite7(&mut self) -> SpriteW<'_, MsigxSpec> { SpriteW::new(self, 7) }
}
/// Sprite X position bit 8; bit n belongs to sprite n
///
/// You can [`read`](crate::Reg::read) this register and get [`msigx::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`msigx::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct MsigxSpec;
impl crate::RegisterSpec for MsigxSpec {
    type Ux = u8;
}
/// `read()` method returns [`msigx::R`](R) reader structure
impl crate::Readable for MsigxSpec {}
/// `write(|w| ..)` method takes [`msigx::W`](W) writer structure
impl crate::Writable for MsigxSpec {
    type Safety = crate::Unsafe;
}
/// `reset()` method sets MSIGX to value 0
impl crate::Resettable for MsigxSpec {}
