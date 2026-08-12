/// Register `SPBGPR` reader
pub type R = crate::R<SpbgprSpec>;
/// Register `SPBGPR` writer
pub type W = crate::W<SpbgprSpec>;
/// Sprite %s drawn behind background (1 = behind)
///
/// Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Priority {
    /// 0: `0`
    InFront = 0,
    /// 1: `1`
    Behind = 1,
}
impl From<Priority> for bool {
    #[inline(always)]
    fn from(variant: Priority) -> Self { variant as u8 != 0 }
}
/// Field `SPRITE(0-7)` reader - Sprite %s drawn behind background (1 = behind)
pub type SpriteR = crate::BitReader<Priority>;
impl SpriteR {
    /// Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Priority {
        match self.bits {
            false => Priority::InFront,
            true => Priority::Behind,
        }
    }

    /// `0`
    #[inline(always)]
    pub fn is_in_front(&self) -> bool { *self == Priority::InFront }

    /// `1`
    #[inline(always)]
    pub fn is_behind(&self) -> bool { *self == Priority::Behind }
}
/// Field `SPRITE(0-7)` writer - Sprite %s drawn behind background (1 = behind)
pub type SpriteW<'a, REG> = crate::BitWriter<'a, REG, Priority>;
impl<'a, REG> SpriteW<'a, REG> where REG: crate::Writable + crate::RegisterSpec, {
    /// `0`
    #[inline(always)]
    pub fn in_front(self) -> &'a mut crate::W<REG> { self.variant(Priority::InFront) }

    /// `1`
    #[inline(always)]
    pub fn behind(self) -> &'a mut crate::W<REG> { self.variant(Priority::Behind) }
}
impl R {
    /// Sprite (0-7) drawn behind background (1 = behind)
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
    /// Sprite (0-7) drawn behind background (1 = behind)
    #[inline(always)]
    pub fn sprite_iter(&self) -> impl Iterator<Item = SpriteR> + '_ {
        (0..8).map(move |n| SpriteR::new(((self.bits >> n) & 1) != 0))
    }

    /// Bit 0 - Sprite 0 drawn behind background (1 = behind)
    #[inline(always)]
    pub fn sprite0(&self) -> SpriteR { SpriteR::new((self.bits & 1) != 0) }

    /// Bit 1 - Sprite 1 drawn behind background (1 = behind)
    #[inline(always)]
    pub fn sprite1(&self) -> SpriteR { SpriteR::new(((self.bits >> 1) & 1) != 0) }

    /// Bit 2 - Sprite 2 drawn behind background (1 = behind)
    #[inline(always)]
    pub fn sprite2(&self) -> SpriteR { SpriteR::new(((self.bits >> 2) & 1) != 0) }

    /// Bit 3 - Sprite 3 drawn behind background (1 = behind)
    #[inline(always)]
    pub fn sprite3(&self) -> SpriteR { SpriteR::new(((self.bits >> 3) & 1) != 0) }

    /// Bit 4 - Sprite 4 drawn behind background (1 = behind)
    #[inline(always)]
    pub fn sprite4(&self) -> SpriteR { SpriteR::new(((self.bits >> 4) & 1) != 0) }

    /// Bit 5 - Sprite 5 drawn behind background (1 = behind)
    #[inline(always)]
    pub fn sprite5(&self) -> SpriteR { SpriteR::new(((self.bits >> 5) & 1) != 0) }

    /// Bit 6 - Sprite 6 drawn behind background (1 = behind)
    #[inline(always)]
    pub fn sprite6(&self) -> SpriteR { SpriteR::new(((self.bits >> 6) & 1) != 0) }

    /// Bit 7 - Sprite 7 drawn behind background (1 = behind)
    #[inline(always)]
    pub fn sprite7(&self) -> SpriteR { SpriteR::new(((self.bits >> 7) & 1) != 0) }
}
impl W {
    /// Sprite (0-7) drawn behind background (1 = behind)
    ///
    /// <div class="warning">`n` is number of field in register. `n == 0`
    /// corresponds to `SPRITE0` field.</div>
    #[inline(always)]
    pub fn sprite(&mut self, n: u8) -> SpriteW<'_, SpbgprSpec> {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        SpriteW::new(self, n)
    }

    /// Bit 0 - Sprite 0 drawn behind background (1 = behind)
    #[inline(always)]
    pub fn sprite0(&mut self) -> SpriteW<'_, SpbgprSpec> { SpriteW::new(self, 0) }

    /// Bit 1 - Sprite 1 drawn behind background (1 = behind)
    #[inline(always)]
    pub fn sprite1(&mut self) -> SpriteW<'_, SpbgprSpec> { SpriteW::new(self, 1) }

    /// Bit 2 - Sprite 2 drawn behind background (1 = behind)
    #[inline(always)]
    pub fn sprite2(&mut self) -> SpriteW<'_, SpbgprSpec> { SpriteW::new(self, 2) }

    /// Bit 3 - Sprite 3 drawn behind background (1 = behind)
    #[inline(always)]
    pub fn sprite3(&mut self) -> SpriteW<'_, SpbgprSpec> { SpriteW::new(self, 3) }

    /// Bit 4 - Sprite 4 drawn behind background (1 = behind)
    #[inline(always)]
    pub fn sprite4(&mut self) -> SpriteW<'_, SpbgprSpec> { SpriteW::new(self, 4) }

    /// Bit 5 - Sprite 5 drawn behind background (1 = behind)
    #[inline(always)]
    pub fn sprite5(&mut self) -> SpriteW<'_, SpbgprSpec> { SpriteW::new(self, 5) }

    /// Bit 6 - Sprite 6 drawn behind background (1 = behind)
    #[inline(always)]
    pub fn sprite6(&mut self) -> SpriteW<'_, SpbgprSpec> { SpriteW::new(self, 6) }

    /// Bit 7 - Sprite 7 drawn behind background (1 = behind)
    #[inline(always)]
    pub fn sprite7(&mut self) -> SpriteW<'_, SpbgprSpec> { SpriteW::new(self, 7) }
}
/// Sprite-to-background priority, bit n = sprite n (1 = background in front)
///
/// You can [`read`](crate::Reg::read) this register and get [`spbgpr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spbgpr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct SpbgprSpec;
impl crate::RegisterSpec for SpbgprSpec {
    type Ux = u8;
}
/// `read()` method returns [`spbgpr::R`](R) reader structure
impl crate::Readable for SpbgprSpec {}
/// `write(|w| ..)` method takes [`spbgpr::W`](W) writer structure
impl crate::Writable for SpbgprSpec {
    type Safety = crate::Unsafe;
}
/// `reset()` method sets SPBGPR to value 0
impl crate::Resettable for SpbgprSpec {}
