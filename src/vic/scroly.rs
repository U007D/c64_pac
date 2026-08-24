/// Register `SCROLY` reader
pub type R = crate::R<ScrolySpec>;
/// Register `SCROLY` writer
pub type W = crate::W<ScrolySpec>;
/// Field `YSCROLL` reader - Vertical fine scroll, and the Bad Line selector. A Bad Line is a raster line on which the VIC-II stalls the CPU to fetch the next row of screen and colour data; the first of each frame is raster 48 + YSCROLL, and that is the line character row 0 starts on. ROW_SELECT sets the first line of the display window - everything on screen that is not border - so only one YSCROLL puts row 0 on that line; the rest slide the character grid and clip the top row behind the border. That sliding is the scroll. ([Bauer, The MOS 6567/6569 video controller (VIC-II)](https://www.cebix.net/VIC-Article.txt), 3.4, 3.7 and 3.9.)
pub type YscrollR = crate::FieldReader;
/// Field `YSCROLL` writer - Vertical fine scroll, and the Bad Line selector. A Bad Line is a raster line on which the VIC-II stalls the CPU to fetch the next row of screen and colour data; the first of each frame is raster 48 + YSCROLL, and that is the line character row 0 starts on. ROW_SELECT sets the first line of the display window - everything on screen that is not border - so only one YSCROLL puts row 0 on that line; the rest slide the character grid and clip the top row behind the border. That sliding is the scroll. ([Bauer, The MOS 6567/6569 video controller (VIC-II)](https://www.cebix.net/VIC-Article.txt), 3.4, 3.7 and 3.9.)
pub type YscrollW<'a, REG> = crate::FieldWriter<'a, REG, 3, u8, crate::Safe>;
/// Height of the display window - everything on screen that is not border. Rows25 spans raster 51-250, Rows24 only 55-246; switching to Rows24 mid-frame is how the top and bottom borders are opened. Fixes which YSCROLL the display needs - see YSCROLL. ([Bauer, The MOS 6567/6569 video controller (VIC-II)](https://www.cebix.net/VIC-Article.txt), 3.4 and 3.9.)
///
/// Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RowSelect {
    /// 0: `0`
    Rows24 = 0,
    /// 1: `1`
    Rows25 = 1,
}
impl From<RowSelect> for bool {
    #[inline(always)]
    fn from(variant: RowSelect) -> Self { variant as u8 != 0 }
}
/// Field `ROW_SELECT` reader - Height of the display window - everything on screen that is not border. Rows25 spans raster 51-250, Rows24 only 55-246; switching to Rows24 mid-frame is how the top and bottom borders are opened. Fixes which YSCROLL the display needs - see YSCROLL. ([Bauer, The MOS 6567/6569 video controller (VIC-II)](https://www.cebix.net/VIC-Article.txt), 3.4 and 3.9.)
pub type RowSelectR = crate::BitReader<RowSelect>;
impl RowSelectR {
    /// Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RowSelect {
        match self.bits {
            false => RowSelect::Rows24,
            true => RowSelect::Rows25,
        }
    }

    /// `0`
    #[inline(always)]
    pub fn is_rows24(&self) -> bool { *self == RowSelect::Rows24 }

    /// `1`
    #[inline(always)]
    pub fn is_rows25(&self) -> bool { *self == RowSelect::Rows25 }
}
/// Field `ROW_SELECT` writer - Height of the display window - everything on screen that is not border. Rows25 spans raster 51-250, Rows24 only 55-246; switching to Rows24 mid-frame is how the top and bottom borders are opened. Fixes which YSCROLL the display needs - see YSCROLL. ([Bauer, The MOS 6567/6569 video controller (VIC-II)](https://www.cebix.net/VIC-Article.txt), 3.4 and 3.9.)
pub type RowSelectW<'a, REG> = crate::BitWriter<'a, REG, RowSelect>;
impl<'a, REG> RowSelectW<'a, REG> where REG: crate::Writable + crate::RegisterSpec, {
    /// `0`
    #[inline(always)]
    pub fn rows24(self) -> &'a mut crate::W<REG> { self.variant(RowSelect::Rows24) }

    /// `1`
    #[inline(always)]
    pub fn rows25(self) -> &'a mut crate::W<REG> { self.variant(RowSelect::Rows25) }
}
/// Enable the display; Disabled blanks the screen to border color
///
/// Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enable {
    /// 0: `0`
    Disabled = 0,
    /// 1: `1`
    Enabled = 1,
}
impl From<Enable> for bool {
    #[inline(always)]
    fn from(variant: Enable) -> Self { variant as u8 != 0 }
}
/// Field `SCREEN` reader - Enable the display; Disabled blanks the screen to
/// border color
///
/// **Shared field type.** The same writer is reused for the fields below;
/// each keeps its own description — click through to read it in context:
/// - [`cia1::ciacra::SelectTimerOutputR`](crate::cia1::ciacra::SelectTimerOutputR)
/// - [`cia1::ciacra::StartTimerR`](crate::cia1::ciacra::StartTimerR)
/// - [`cia1::ciacrb::SelectTimerOutputR`](crate::cia1::ciacrb::SelectTimerOutputR)
/// - [`cia1::ciacrb::StartTimerR`](crate::cia1::ciacrb::StartTimerR)
/// - [`cia2::ci2cra::SelectTimerOutputR`](crate::cia2::ci2cra::SelectTimerOutputR)
/// - [`cia2::ci2cra::StartTimerR`](crate::cia2::ci2cra::StartTimerR)
/// - [`cia2::ci2crb::SelectTimerOutputR`](crate::cia2::ci2crb::SelectTimerOutputR)
/// - [`cia2::ci2crb::StartTimerR`](crate::cia2::ci2crb::StartTimerR)
/// - [`vic::irqmask::LightPenR`](crate::vic::irqmask::LightPenR)
/// - [`vic::irqmask::RasterR`](crate::vic::irqmask::RasterR)
/// - [`vic::irqmask::SpriteBgCollisionR`](crate::vic::irqmask::SpriteBgCollisionR)
/// - [`vic::irqmask::SpriteSpriteCollisionR`](crate::vic::irqmask::SpriteSpriteCollisionR)
/// - [`vic::scrolx::MulticolorModeR`](crate::vic::scrolx::MulticolorModeR)
/// - [`vic::scroly::BitmapModeR`](crate::vic::scroly::BitmapModeR)
/// - [`vic::scroly::ExtendedColorModeR`](crate::vic::scroly::ExtendedColorModeR)
/// - [`vic::spena::SpriteR`](crate::vic::spena::SpriteR)
/// - [`vic::spmc::SpriteR`](crate::vic::spmc::SpriteR)
/// - [`vic::xxpand::SpriteR`](crate::vic::xxpand::SpriteR)
/// - [`vic::yxpand::SpriteR`](crate::vic::yxpand::SpriteR)
pub type ScreenR = crate::BitReader<Enable>;
impl ScreenR {
    /// Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Enable {
        match self.bits {
            false => Enable::Disabled,
            true => Enable::Enabled,
        }
    }

    /// `0`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool { *self == Enable::Disabled }

    /// `1`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool { *self == Enable::Enabled }
}
/// Field `SCREEN` writer - Enable the display; Disabled blanks the screen to
/// border color
///
/// **Shared field type.** The same writer is reused for the fields below;
/// each keeps its own description — click through to read it in context:
/// - [`cia1::ciacra::SelectTimerOutputW`](crate::cia1::ciacra::SelectTimerOutputW)
/// - [`cia1::ciacra::StartTimerW`](crate::cia1::ciacra::StartTimerW)
/// - [`cia1::ciacrb::SelectTimerOutputW`](crate::cia1::ciacrb::SelectTimerOutputW)
/// - [`cia1::ciacrb::StartTimerW`](crate::cia1::ciacrb::StartTimerW)
/// - [`cia1::ciaicr_w::ModeW`](crate::cia1::ciaicr_w::ModeW)
/// - [`cia2::ci2cra::SelectTimerOutputW`](crate::cia2::ci2cra::SelectTimerOutputW)
/// - [`cia2::ci2cra::StartTimerW`](crate::cia2::ci2cra::StartTimerW)
/// - [`cia2::ci2crb::SelectTimerOutputW`](crate::cia2::ci2crb::SelectTimerOutputW)
/// - [`cia2::ci2crb::StartTimerW`](crate::cia2::ci2crb::StartTimerW)
/// - [`cia2::ci2icr_w::ModeW`](crate::cia2::ci2icr_w::ModeW)
/// - [`sid::reson::Filt1W`](crate::sid::reson::Filt1W)
/// - [`sid::reson::Filt2W`](crate::sid::reson::Filt2W)
/// - [`sid::reson::Filt3W`](crate::sid::reson::Filt3W)
/// - [`sid::reson::FiltExtW`](crate::sid::reson::FiltExtW)
/// - [`sid::sigvol::BandPassW`](crate::sid::sigvol::BandPassW)
/// - [`sid::sigvol::HighPassW`](crate::sid::sigvol::HighPassW)
/// - [`sid::sigvol::LowPassW`](crate::sid::sigvol::LowPassW)
/// - [`sid::voice::vcreg::NoiseW`](crate::sid::voice::vcreg::NoiseW)
/// - [`sid::voice::vcreg::PulseW`](crate::sid::voice::vcreg::PulseW)
/// - [`sid::voice::vcreg::RingW`](crate::sid::voice::vcreg::RingW)
/// - [`sid::voice::vcreg::SawtoothW`](crate::sid::voice::vcreg::SawtoothW)
/// - [`sid::voice::vcreg::SyncW`](crate::sid::voice::vcreg::SyncW)
/// - [`sid::voice::vcreg::TriangleW`](crate::sid::voice::vcreg::TriangleW)
/// - [`vic::irqmask::LightPenW`](crate::vic::irqmask::LightPenW)
/// - [`vic::irqmask::RasterW`](crate::vic::irqmask::RasterW)
/// - [`vic::irqmask::SpriteBgCollisionW`](crate::vic::irqmask::SpriteBgCollisionW)
/// - [`vic::irqmask::SpriteSpriteCollisionW`](crate::vic::irqmask::SpriteSpriteCollisionW)
/// - [`vic::scrolx::MulticolorModeW`](crate::vic::scrolx::MulticolorModeW)
/// - [`vic::scroly::BitmapModeW`](crate::vic::scroly::BitmapModeW)
/// - [`vic::scroly::ExtendedColorModeW`](crate::vic::scroly::ExtendedColorModeW)
/// - [`vic::spena::SpriteW`](crate::vic::spena::SpriteW)
/// - [`vic::spmc::SpriteW`](crate::vic::spmc::SpriteW)
/// - [`vic::xxpand::SpriteW`](crate::vic::xxpand::SpriteW)
/// - [`vic::yxpand::SpriteW`](crate::vic::yxpand::SpriteW)
pub type ScreenW<'a, REG> = crate::BitWriter<'a, REG, Enable>;
impl<'a, REG> ScreenW<'a, REG> where REG: crate::Writable + crate::RegisterSpec, {
    /// `0`
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> { self.variant(Enable::Disabled) }

    /// `1`
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> { self.variant(Enable::Enabled) }
}
/// Field `BITMAP_MODE` reader - Bitmap graphics mode
pub use ScreenR as BitmapModeR;
/// Field `EXTENDED_COLOR_MODE` reader - Extended background color mode
pub use ScreenR as ExtendedColorModeR;
/// Field `BITMAP_MODE` writer - Bitmap graphics mode
pub use ScreenW as BitmapModeW;
/// Field `EXTENDED_COLOR_MODE` writer - Extended background color mode
pub use ScreenW as ExtendedColorModeW;
/// Field `RST8` reader - Raster bit 8 (read: current line; write: raster-IRQ
/// compare)
pub type Rst8R = crate::BitReader;
/// Field `RST8` writer - Raster bit 8 (read: current line; write: raster-IRQ
/// compare)
pub type Rst8W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    /// Bits 0:2 - Vertical fine scroll, and the Bad Line selector. A Bad Line is a raster line on which the VIC-II stalls the CPU to fetch the next row of screen and colour data; the first of each frame is raster 48 + YSCROLL, and that is the line character row 0 starts on. ROW_SELECT sets the first line of the display window - everything on screen that is not border - so only one YSCROLL puts row 0 on that line; the rest slide the character grid and clip the top row behind the border. That sliding is the scroll. ([Bauer, The MOS 6567/6569 video controller (VIC-II)](https://www.cebix.net/VIC-Article.txt), 3.4, 3.7 and 3.9.)
    ///
    /// | `ROW_SELECT` | first display line | `YSCROLL` | first Bad Line |
    /// |---|---|---|---|
    /// | `Rows25` | raster 51 | 3 | 48 + 3 = 51 |
    /// | `Rows24` | raster 55 | 7 | 48 + 7 = 55 |
    #[inline(always)]
    pub fn yscroll(&self) -> YscrollR { YscrollR::new(self.bits & 7) }

    /// Bit 3 - Height of the display window - everything on screen that is not border. Rows25 spans raster 51-250, Rows24 only 55-246; switching to Rows24 mid-frame is how the top and bottom borders are opened. Fixes which YSCROLL the display needs - see YSCROLL. ([Bauer, The MOS 6567/6569 video controller (VIC-II)](https://www.cebix.net/VIC-Article.txt), 3.4 and 3.9.)
    #[inline(always)]
    pub fn row_select(&self) -> RowSelectR { RowSelectR::new(((self.bits >> 3) & 1) != 0) }

    /// Bit 4 - Enable the display; Disabled blanks the screen to border color
    #[inline(always)]
    pub fn screen(&self) -> ScreenR { ScreenR::new(((self.bits >> 4) & 1) != 0) }

    /// Bit 5 - Bitmap graphics mode
    #[inline(always)]
    pub fn bitmap_mode(&self) -> BitmapModeR { BitmapModeR::new(((self.bits >> 5) & 1) != 0) }

    /// Bit 6 - Extended background color mode
    #[inline(always)]
    pub fn extended_color_mode(&self) -> ExtendedColorModeR {
        ExtendedColorModeR::new(((self.bits >> 6) & 1) != 0)
    }

    /// Bit 7 - Raster bit 8 (read: current line; write: raster-IRQ compare)
    #[inline(always)]
    pub fn rst8(&self) -> Rst8R { Rst8R::new(((self.bits >> 7) & 1) != 0) }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SCROLY")
         .field("yscroll", &self.yscroll())
         .field("row_select", &self.row_select())
         .field("screen", &self.screen())
         .field("bitmap_mode", &self.bitmap_mode())
         .field("extended_color_mode", &self.extended_color_mode())
         .field("rst8", &self.rst8())
         .finish()
    }
}
impl W {
    /// Bits 0:2 - Vertical fine scroll, and the Bad Line selector. A Bad Line is a raster line on which the VIC-II stalls the CPU to fetch the next row of screen and colour data; the first of each frame is raster 48 + YSCROLL, and that is the line character row 0 starts on. ROW_SELECT sets the first line of the display window - everything on screen that is not border - so only one YSCROLL puts row 0 on that line; the rest slide the character grid and clip the top row behind the border. That sliding is the scroll. ([Bauer, The MOS 6567/6569 video controller (VIC-II)](https://www.cebix.net/VIC-Article.txt), 3.4, 3.7 and 3.9.)
    ///
    /// | `ROW_SELECT` | first display line | `YSCROLL` | first Bad Line |
    /// |---|---|---|---|
    /// | `Rows25` | raster 51 | 3 | 48 + 3 = 51 |
    /// | `Rows24` | raster 55 | 7 | 48 + 7 = 55 |
    #[inline(always)]
    pub fn yscroll(&mut self) -> YscrollW<'_, ScrolySpec> { YscrollW::new(self, 0) }

    /// Bit 3 - Height of the display window - everything on screen that is not border. Rows25 spans raster 51-250, Rows24 only 55-246; switching to Rows24 mid-frame is how the top and bottom borders are opened. Fixes which YSCROLL the display needs - see YSCROLL. ([Bauer, The MOS 6567/6569 video controller (VIC-II)](https://www.cebix.net/VIC-Article.txt), 3.4 and 3.9.)
    #[inline(always)]
    pub fn row_select(&mut self) -> RowSelectW<'_, ScrolySpec> { RowSelectW::new(self, 3) }

    /// Bit 4 - Enable the display; Disabled blanks the screen to border color
    #[inline(always)]
    pub fn screen(&mut self) -> ScreenW<'_, ScrolySpec> { ScreenW::new(self, 4) }

    /// Bit 5 - Bitmap graphics mode
    #[inline(always)]
    pub fn bitmap_mode(&mut self) -> BitmapModeW<'_, ScrolySpec> { BitmapModeW::new(self, 5) }

    /// Bit 6 - Extended background color mode
    #[inline(always)]
    pub fn extended_color_mode(&mut self) -> ExtendedColorModeW<'_, ScrolySpec> {
        ExtendedColorModeW::new(self, 6)
    }

    /// Bit 7 - Raster bit 8 (read: current line; write: raster-IRQ compare)
    #[inline(always)]
    pub fn rst8(&mut self) -> Rst8W<'_, ScrolySpec> { Rst8W::new(self, 7) }
}
/// Control register 1. RST8 reads current raster bit 8, writes raster-compare
/// bit 8.
///
/// You can [`read`](crate::Reg::read) this register and get [`scroly::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scroly::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct ScrolySpec;
impl crate::RegisterSpec for ScrolySpec {
    type Ux = u8;
}
/// `read()` method returns [`scroly::R`](R) reader structure
impl crate::Readable for ScrolySpec {}
/// `write(|w| ..)` method takes [`scroly::W`](W) writer structure
impl crate::Writable for ScrolySpec {
    type Safety = crate::Unsafe;
}
/// `reset()` method sets SCROLY to value 0
impl crate::Resettable for ScrolySpec {}
