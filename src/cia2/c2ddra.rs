/// Register `C2DDRA` reader
pub type R = crate::R<C2ddraSpec>;
/// Register `C2DDRA` writer
pub type W = crate::W<C2ddraSpec>;
/// Direction of Data Port A bit 0
///
/// Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Direction {
    /// 0: `0`
    Input = 0,
    /// 1: `1`
    Output = 1,
}
impl From<Direction> for bool {
    #[inline(always)]
    fn from(variant: Direction) -> Self { variant as u8 != 0 }
}
/// Field `LINE0` reader - Direction of Data Port A bit 0
pub type Line0R = crate::BitReader<Direction>;
impl Line0R {
    /// Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Direction {
        match self.bits {
            false => Direction::Input,
            true => Direction::Output,
        }
    }

    /// `0`
    #[inline(always)]
    pub fn is_input(&self) -> bool { *self == Direction::Input }

    /// `1`
    #[inline(always)]
    pub fn is_output(&self) -> bool { *self == Direction::Output }
}
/// Field `LINE0` writer - Direction of Data Port A bit 0
pub type Line0W<'a, REG> = crate::BitWriter<'a, REG, Direction>;
impl<'a, REG> Line0W<'a, REG> where REG: crate::Writable + crate::RegisterSpec, {
    /// `0`
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> { self.variant(Direction::Input) }

    /// `1`
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> { self.variant(Direction::Output) }
}
/// Field `LINE1` reader - Direction of Data Port A bit 1
pub use Line0R as Line1R;
/// Field `LINE2` reader - Direction of Data Port A bit 2
pub use Line0R as Line2R;
/// Field `LINE3` reader - Direction of Data Port A bit 3
pub use Line0R as Line3R;
/// Field `LINE4` reader - Direction of Data Port A bit 4
pub use Line0R as Line4R;
/// Field `LINE5` reader - Direction of Data Port A bit 5
pub use Line0R as Line5R;
/// Field `LINE6` reader - Direction of Data Port A bit 6
pub use Line0R as Line6R;
/// Field `LINE7` reader - Direction of Data Port A bit 7
pub use Line0R as Line7R;
/// Field `LINE1` writer - Direction of Data Port A bit 1
pub use Line0W as Line1W;
/// Field `LINE2` writer - Direction of Data Port A bit 2
pub use Line0W as Line2W;
/// Field `LINE3` writer - Direction of Data Port A bit 3
pub use Line0W as Line3W;
/// Field `LINE4` writer - Direction of Data Port A bit 4
pub use Line0W as Line4W;
/// Field `LINE5` writer - Direction of Data Port A bit 5
pub use Line0W as Line5W;
/// Field `LINE6` writer - Direction of Data Port A bit 6
pub use Line0W as Line6W;
/// Field `LINE7` writer - Direction of Data Port A bit 7
pub use Line0W as Line7W;
impl R {
    /// Bit 0 - Direction of Data Port A bit 0
    #[inline(always)]
    pub fn line0(&self) -> Line0R { Line0R::new((self.bits & 1) != 0) }

    /// Bit 1 - Direction of Data Port A bit 1
    #[inline(always)]
    pub fn line1(&self) -> Line1R { Line1R::new(((self.bits >> 1) & 1) != 0) }

    /// Bit 2 - Direction of Data Port A bit 2
    #[inline(always)]
    pub fn line2(&self) -> Line2R { Line2R::new(((self.bits >> 2) & 1) != 0) }

    /// Bit 3 - Direction of Data Port A bit 3
    #[inline(always)]
    pub fn line3(&self) -> Line3R { Line3R::new(((self.bits >> 3) & 1) != 0) }

    /// Bit 4 - Direction of Data Port A bit 4
    #[inline(always)]
    pub fn line4(&self) -> Line4R { Line4R::new(((self.bits >> 4) & 1) != 0) }

    /// Bit 5 - Direction of Data Port A bit 5
    #[inline(always)]
    pub fn line5(&self) -> Line5R { Line5R::new(((self.bits >> 5) & 1) != 0) }

    /// Bit 6 - Direction of Data Port A bit 6
    #[inline(always)]
    pub fn line6(&self) -> Line6R { Line6R::new(((self.bits >> 6) & 1) != 0) }

    /// Bit 7 - Direction of Data Port A bit 7
    #[inline(always)]
    pub fn line7(&self) -> Line7R { Line7R::new(((self.bits >> 7) & 1) != 0) }
}
impl W {
    /// Bit 0 - Direction of Data Port A bit 0
    #[inline(always)]
    pub fn line0(&mut self) -> Line0W<'_, C2ddraSpec> { Line0W::new(self, 0) }

    /// Bit 1 - Direction of Data Port A bit 1
    #[inline(always)]
    pub fn line1(&mut self) -> Line1W<'_, C2ddraSpec> { Line1W::new(self, 1) }

    /// Bit 2 - Direction of Data Port A bit 2
    #[inline(always)]
    pub fn line2(&mut self) -> Line2W<'_, C2ddraSpec> { Line2W::new(self, 2) }

    /// Bit 3 - Direction of Data Port A bit 3
    #[inline(always)]
    pub fn line3(&mut self) -> Line3W<'_, C2ddraSpec> { Line3W::new(self, 3) }

    /// Bit 4 - Direction of Data Port A bit 4
    #[inline(always)]
    pub fn line4(&mut self) -> Line4W<'_, C2ddraSpec> { Line4W::new(self, 4) }

    /// Bit 5 - Direction of Data Port A bit 5
    #[inline(always)]
    pub fn line5(&mut self) -> Line5W<'_, C2ddraSpec> { Line5W::new(self, 5) }

    /// Bit 6 - Direction of Data Port A bit 6
    #[inline(always)]
    pub fn line6(&mut self) -> Line6W<'_, C2ddraSpec> { Line6W::new(self, 6) }

    /// Bit 7 - Direction of Data Port A bit 7
    #[inline(always)]
    pub fn line7(&mut self) -> Line7W<'_, C2ddraSpec> { Line7W::new(self, 7) }
}
/// Port A data direction (1 = output). KERNAL sets 0x3F.
///
/// You can [`read`](crate::Reg::read) this register and get [`c2ddra::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c2ddra::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct C2ddraSpec;
impl crate::RegisterSpec for C2ddraSpec {
    type Ux = u8;
}
/// `read()` method returns [`c2ddra::R`](R) reader structure
impl crate::Readable for C2ddraSpec {}
/// `write(|w| ..)` method takes [`c2ddra::W`](W) writer structure
impl crate::Writable for C2ddraSpec {
    type Safety = crate::Unsafe;
}
/// `reset()` method sets C2DDRA to value 0
impl crate::Resettable for C2ddraSpec {}
