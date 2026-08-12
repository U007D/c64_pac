/// Register `SPBGCL` reader
pub type R = crate::R<SpbgclSpec>;
/// Sprite %s collided with the background
///
/// Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Collision {
    /// 0: `0`
    Clear = 0,
    /// 1: `1`
    Collided = 1,
}
impl From<Collision> for bool {
    #[inline(always)]
    fn from(variant: Collision) -> Self { variant as u8 != 0 }
}
/// Field `SPRITE(0-7)` reader - Sprite %s collided with the background
pub type SpriteR = crate::BitReader<Collision>;
impl SpriteR {
    /// Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Collision {
        match self.bits {
            false => Collision::Clear,
            true => Collision::Collided,
        }
    }

    /// `0`
    #[inline(always)]
    pub fn is_clear(&self) -> bool { *self == Collision::Clear }

    /// `1`
    #[inline(always)]
    pub fn is_collided(&self) -> bool { *self == Collision::Collided }
}
impl R {
    /// Sprite (0-7) collided with the background
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
    /// Sprite (0-7) collided with the background
    #[inline(always)]
    pub fn sprite_iter(&self) -> impl Iterator<Item = SpriteR> + '_ {
        (0..8).map(move |n| SpriteR::new(((self.bits >> n) & 1) != 0))
    }

    /// Bit 0 - Sprite 0 collided with the background
    #[inline(always)]
    pub fn sprite0(&self) -> SpriteR { SpriteR::new((self.bits & 1) != 0) }

    /// Bit 1 - Sprite 1 collided with the background
    #[inline(always)]
    pub fn sprite1(&self) -> SpriteR { SpriteR::new(((self.bits >> 1) & 1) != 0) }

    /// Bit 2 - Sprite 2 collided with the background
    #[inline(always)]
    pub fn sprite2(&self) -> SpriteR { SpriteR::new(((self.bits >> 2) & 1) != 0) }

    /// Bit 3 - Sprite 3 collided with the background
    #[inline(always)]
    pub fn sprite3(&self) -> SpriteR { SpriteR::new(((self.bits >> 3) & 1) != 0) }

    /// Bit 4 - Sprite 4 collided with the background
    #[inline(always)]
    pub fn sprite4(&self) -> SpriteR { SpriteR::new(((self.bits >> 4) & 1) != 0) }

    /// Bit 5 - Sprite 5 collided with the background
    #[inline(always)]
    pub fn sprite5(&self) -> SpriteR { SpriteR::new(((self.bits >> 5) & 1) != 0) }

    /// Bit 6 - Sprite 6 collided with the background
    #[inline(always)]
    pub fn sprite6(&self) -> SpriteR { SpriteR::new(((self.bits >> 6) & 1) != 0) }

    /// Bit 7 - Sprite 7 collided with the background
    #[inline(always)]
    pub fn sprite7(&self) -> SpriteR { SpriteR::new(((self.bits >> 7) & 1) != 0) }
}
/// Sprite-background collision latch. SIDE-EFFECT: reading clears the latch -
/// read once and keep the value.
///
/// You can [`read`](crate::Reg::read) this register and get [`spbgcl::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
/// <div class="warning">The register is <b>cleared</b> (set to zero) following
/// a read operation.</div>
pub struct SpbgclSpec;
impl crate::RegisterSpec for SpbgclSpec {
    type Ux = u8;
}
/// `read()` method returns [`spbgcl::R`](R) reader structure
impl crate::Readable for SpbgclSpec {}
/// `reset()` method sets SPBGCL to value 0
impl crate::Resettable for SpbgclSpec {}
