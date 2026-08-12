/// Register `EXTCOL` reader
pub type R = crate::R<ExtcolSpec>;
/// Register `EXTCOL` writer
pub type W = crate::W<ExtcolSpec>;
/// Border color
///
/// Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
    /// 0: `0`
    Black = 0,
    /// 1: `1`
    White = 1,
    /// 2: `10`
    Red = 2,
    /// 3: `11`
    Cyan = 3,
    /// 4: `100`
    Purple = 4,
    /// 5: `101`
    Green = 5,
    /// 6: `110`
    Blue = 6,
    /// 7: `111`
    Yellow = 7,
    /// 8: `1000`
    Orange = 8,
    /// 9: `1001`
    Brown = 9,
    /// 10: `1010`
    LightRed = 10,
    /// 11: `1011`
    DarkGray = 11,
    /// 12: `1100`
    MediumGray = 12,
    /// 13: `1101`
    LightGreen = 13,
    /// 14: `1110`
    LightBlue = 14,
    /// 15: `1111`
    LightGray = 15,
}
impl From<Color> for u8 {
    #[inline(always)]
    fn from(variant: Color) -> Self { variant as _ }
}
impl crate::FieldSpec for Color {
    type Ux = u8;
}
impl crate::IsEnum for Color {}
/// Field `COLOR` reader - Border color
pub type ColorR = crate::FieldReader<Color>;
impl ColorR {
    /// Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Color {
        match self.bits {
            0 => Color::Black,
            1 => Color::White,
            2 => Color::Red,
            3 => Color::Cyan,
            4 => Color::Purple,
            5 => Color::Green,
            6 => Color::Blue,
            7 => Color::Yellow,
            8 => Color::Orange,
            9 => Color::Brown,
            10 => Color::LightRed,
            11 => Color::DarkGray,
            12 => Color::MediumGray,
            13 => Color::LightGreen,
            14 => Color::LightBlue,
            15 => Color::LightGray,
            _ => unreachable!(),
        }
    }

    /// `0`
    #[inline(always)]
    pub fn is_black(&self) -> bool { *self == Color::Black }

    /// `1`
    #[inline(always)]
    pub fn is_white(&self) -> bool { *self == Color::White }

    /// `10`
    #[inline(always)]
    pub fn is_red(&self) -> bool { *self == Color::Red }

    /// `11`
    #[inline(always)]
    pub fn is_cyan(&self) -> bool { *self == Color::Cyan }

    /// `100`
    #[inline(always)]
    pub fn is_purple(&self) -> bool { *self == Color::Purple }

    /// `101`
    #[inline(always)]
    pub fn is_green(&self) -> bool { *self == Color::Green }

    /// `110`
    #[inline(always)]
    pub fn is_blue(&self) -> bool { *self == Color::Blue }

    /// `111`
    #[inline(always)]
    pub fn is_yellow(&self) -> bool { *self == Color::Yellow }

    /// `1000`
    #[inline(always)]
    pub fn is_orange(&self) -> bool { *self == Color::Orange }

    /// `1001`
    #[inline(always)]
    pub fn is_brown(&self) -> bool { *self == Color::Brown }

    /// `1010`
    #[inline(always)]
    pub fn is_light_red(&self) -> bool { *self == Color::LightRed }

    /// `1011`
    #[inline(always)]
    pub fn is_dark_gray(&self) -> bool { *self == Color::DarkGray }

    /// `1100`
    #[inline(always)]
    pub fn is_medium_gray(&self) -> bool { *self == Color::MediumGray }

    /// `1101`
    #[inline(always)]
    pub fn is_light_green(&self) -> bool { *self == Color::LightGreen }

    /// `1110`
    #[inline(always)]
    pub fn is_light_blue(&self) -> bool { *self == Color::LightBlue }

    /// `1111`
    #[inline(always)]
    pub fn is_light_gray(&self) -> bool { *self == Color::LightGray }
}
/// Field `COLOR` writer - Border color
pub type ColorW<'a, REG> = crate::FieldWriter<'a, REG, 4, Color, crate::Safe>;
impl<'a, REG> ColorW<'a, REG>
    where REG: crate::Writable + crate::RegisterSpec,
          REG::Ux: From<u8>,
{
    /// `0`
    #[inline(always)]
    pub fn black(self) -> &'a mut crate::W<REG> { self.variant(Color::Black) }

    /// `1`
    #[inline(always)]
    pub fn white(self) -> &'a mut crate::W<REG> { self.variant(Color::White) }

    /// `10`
    #[inline(always)]
    pub fn red(self) -> &'a mut crate::W<REG> { self.variant(Color::Red) }

    /// `11`
    #[inline(always)]
    pub fn cyan(self) -> &'a mut crate::W<REG> { self.variant(Color::Cyan) }

    /// `100`
    #[inline(always)]
    pub fn purple(self) -> &'a mut crate::W<REG> { self.variant(Color::Purple) }

    /// `101`
    #[inline(always)]
    pub fn green(self) -> &'a mut crate::W<REG> { self.variant(Color::Green) }

    /// `110`
    #[inline(always)]
    pub fn blue(self) -> &'a mut crate::W<REG> { self.variant(Color::Blue) }

    /// `111`
    #[inline(always)]
    pub fn yellow(self) -> &'a mut crate::W<REG> { self.variant(Color::Yellow) }

    /// `1000`
    #[inline(always)]
    pub fn orange(self) -> &'a mut crate::W<REG> { self.variant(Color::Orange) }

    /// `1001`
    #[inline(always)]
    pub fn brown(self) -> &'a mut crate::W<REG> { self.variant(Color::Brown) }

    /// `1010`
    #[inline(always)]
    pub fn light_red(self) -> &'a mut crate::W<REG> { self.variant(Color::LightRed) }

    /// `1011`
    #[inline(always)]
    pub fn dark_gray(self) -> &'a mut crate::W<REG> { self.variant(Color::DarkGray) }

    /// `1100`
    #[inline(always)]
    pub fn medium_gray(self) -> &'a mut crate::W<REG> { self.variant(Color::MediumGray) }

    /// `1101`
    #[inline(always)]
    pub fn light_green(self) -> &'a mut crate::W<REG> { self.variant(Color::LightGreen) }

    /// `1110`
    #[inline(always)]
    pub fn light_blue(self) -> &'a mut crate::W<REG> { self.variant(Color::LightBlue) }

    /// `1111`
    #[inline(always)]
    pub fn light_gray(self) -> &'a mut crate::W<REG> { self.variant(Color::LightGray) }
}
impl R {
    /// Bits 0:3 - Border color
    #[inline(always)]
    pub fn color(&self) -> ColorR { ColorR::new(self.bits & 0x0f) }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EXTCOL").field("color", &self.color()).finish()
    }
}
impl W {
    /// Bits 0:3 - Border color
    #[inline(always)]
    pub fn color(&mut self) -> ColorW<'_, ExtcolSpec> { ColorW::new(self, 0) }
}
/// Border color (bits 3:0; upper bits read as 1)
///
/// You can [`read`](crate::Reg::read) this register and get [`extcol::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`extcol::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct ExtcolSpec;
impl crate::RegisterSpec for ExtcolSpec {
    type Ux = u8;
}
/// `read()` method returns [`extcol::R`](R) reader structure
impl crate::Readable for ExtcolSpec {}
/// `write(|w| ..)` method takes [`extcol::W`](W) writer structure
impl crate::Writable for ExtcolSpec {
    type Safety = crate::Unsafe;
}
/// `reset()` method sets EXTCOL to value 0
impl crate::Resettable for ExtcolSpec {}
