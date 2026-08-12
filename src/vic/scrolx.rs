/// Register `SCROLX` reader
pub type R = crate::R<ScrolxSpec>;
/// Register `SCROLX` writer
pub type W = crate::W<ScrolxSpec>;
/// Field `XSCROLL` reader - Horizontal fine scroll (0-7)
pub type XscrollR = crate::FieldReader;
/// Field `XSCROLL` writer - Horizontal fine scroll (0-7)
pub type XscrollW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
/// Text area width: 38 or 40 columns
///
/// Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ColumnSelect {
    /// 0: `0`
    Col38 = 0,
    /// 1: `1`
    Col40 = 1,
}
impl From<ColumnSelect> for bool {
    #[inline(always)]
    fn from(variant: ColumnSelect) -> Self { variant as u8 != 0 }
}
/// Field `COLUMN_SELECT` reader - Text area width: 38 or 40 columns
pub type ColumnSelectR = crate::BitReader<ColumnSelect>;
impl ColumnSelectR {
    /// Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ColumnSelect {
        match self.bits {
            false => ColumnSelect::Col38,
            true => ColumnSelect::Col40,
        }
    }

    /// `0`
    #[inline(always)]
    pub fn is_col38(&self) -> bool { *self == ColumnSelect::Col38 }

    /// `1`
    #[inline(always)]
    pub fn is_col40(&self) -> bool { *self == ColumnSelect::Col40 }
}
/// Field `COLUMN_SELECT` writer - Text area width: 38 or 40 columns
pub type ColumnSelectW<'a, REG> = crate::BitWriter<'a, REG, ColumnSelect>;
impl<'a, REG> ColumnSelectW<'a, REG> where REG: crate::Writable + crate::RegisterSpec, {
    /// `0`
    #[inline(always)]
    pub fn col38(self) -> &'a mut crate::W<REG> { self.variant(ColumnSelect::Col38) }

    /// `1`
    #[inline(always)]
    pub fn col40(self) -> &'a mut crate::W<REG> { self.variant(ColumnSelect::Col40) }
}
/// Multicolor mode
pub use super::scroly::Enable;
/// Field `MULTICOLOR_MODE` reader - Multicolor mode
pub use super::scroly::ScreenR as MulticolorModeR;
/// Field `MULTICOLOR_MODE` writer - Multicolor mode
pub use super::scroly::ScreenW as MulticolorModeW;
/// Field `RES` reader - No function on 6567/6569
pub type ResR = crate::BitReader;
/// Field `RES` writer - No function on 6567/6569
pub type ResW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    /// Bits 0:2 - Horizontal fine scroll (0-7)
    #[inline(always)]
    pub fn xscroll(&self) -> XscrollR { XscrollR::new(self.bits & 7) }

    /// Bit 3 - Text area width: 38 or 40 columns
    #[inline(always)]
    pub fn column_select(&self) -> ColumnSelectR { ColumnSelectR::new(((self.bits >> 3) & 1) != 0) }

    /// Bit 4 - Multicolor mode
    #[inline(always)]
    pub fn multicolor_mode(&self) -> MulticolorModeR {
        MulticolorModeR::new(((self.bits >> 4) & 1) != 0)
    }

    /// Bit 5 - No function on 6567/6569
    #[inline(always)]
    pub fn res(&self) -> ResR { ResR::new(((self.bits >> 5) & 1) != 0) }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SCROLX")
         .field("xscroll", &self.xscroll())
         .field("column_select", &self.column_select())
         .field("multicolor_mode", &self.multicolor_mode())
         .field("res", &self.res())
         .finish()
    }
}
impl W {
    /// Bits 0:2 - Horizontal fine scroll (0-7)
    #[inline(always)]
    pub fn xscroll(&mut self) -> XscrollW<'_, ScrolxSpec> { XscrollW::new(self, 0) }

    /// Bit 3 - Text area width: 38 or 40 columns
    #[inline(always)]
    pub fn column_select(&mut self) -> ColumnSelectW<'_, ScrolxSpec> { ColumnSelectW::new(self, 3) }

    /// Bit 4 - Multicolor mode
    #[inline(always)]
    pub fn multicolor_mode(&mut self) -> MulticolorModeW<'_, ScrolxSpec> {
        MulticolorModeW::new(self, 4)
    }

    /// Bit 5 - No function on 6567/6569
    #[inline(always)]
    pub fn res(&mut self) -> ResW<'_, ScrolxSpec> { ResW::new(self, 5) }
}
/// Control register 2. Bits 6-7 unimplemented, read as 1.
///
/// You can [`read`](crate::Reg::read) this register and get [`scrolx::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scrolx::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct ScrolxSpec;
impl crate::RegisterSpec for ScrolxSpec {
    type Ux = u8;
}
/// `read()` method returns [`scrolx::R`](R) reader structure
impl crate::Readable for ScrolxSpec {}
/// `write(|w| ..)` method takes [`scrolx::W`](W) writer structure
impl crate::Writable for ScrolxSpec {
    type Safety = crate::Unsafe;
}
/// `reset()` method sets SCROLX to value 0
impl crate::Resettable for ScrolxSpec {}
