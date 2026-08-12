/// Register `CIAPRA` reader
pub type R = crate::R<CiapraSpec>;
/// Register `CIAPRA` writer
pub type W = crate::W<CiapraSpec>;
/// Joystick 2 up (also keyboard column 0). Active low.
///
/// Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Direction {
    /// 0: `0`
    Pushed = 0,
    /// 1: `1`
    Released = 1,
}
impl From<Direction> for bool {
    #[inline(always)]
    fn from(variant: Direction) -> Self { variant as u8 != 0 }
}
/// Field `UP` reader - Joystick 2 up (also keyboard column 0). Active low.
pub type UpR = crate::BitReader<Direction>;
impl UpR {
    /// Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Direction {
        match self.bits {
            false => Direction::Pushed,
            true => Direction::Released,
        }
    }

    /// `0`
    #[inline(always)]
    pub fn is_pushed(&self) -> bool { *self == Direction::Pushed }

    /// `1`
    #[inline(always)]
    pub fn is_released(&self) -> bool { *self == Direction::Released }
}
/// Field `UP` writer - Joystick 2 up (also keyboard column 0). Active low.
pub type UpW<'a, REG> = crate::BitWriter<'a, REG, Direction>;
impl<'a, REG> UpW<'a, REG> where REG: crate::Writable + crate::RegisterSpec, {
    /// `0`
    #[inline(always)]
    pub fn pushed(self) -> &'a mut crate::W<REG> { self.variant(Direction::Pushed) }

    /// `1`
    #[inline(always)]
    pub fn released(self) -> &'a mut crate::W<REG> { self.variant(Direction::Released) }
}
/// Field `DOWN` reader - Joystick 2 down (also keyboard column 1). Active low.
pub use UpR as DownR;
/// Field `LEFT` reader - Joystick 2 left (also keyboard column 2). Active low.
pub use UpR as LeftR;
/// Field `RIGHT` reader - Joystick 2 right (also keyboard column 3). Active
/// low.
pub use UpR as RightR;
/// Field `DOWN` writer - Joystick 2 down (also keyboard column 1). Active low.
pub use UpW as DownW;
/// Field `LEFT` writer - Joystick 2 left (also keyboard column 2). Active low.
pub use UpW as LeftW;
/// Field `RIGHT` writer - Joystick 2 right (also keyboard column 3). Active
/// low.
pub use UpW as RightW;
/// Joystick 2 fire (also keyboard column 4). Active low.
///
/// Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Button {
    /// 0: `0`
    Pressed = 0,
    /// 1: `1`
    Released = 1,
}
impl From<Button> for bool {
    #[inline(always)]
    fn from(variant: Button) -> Self { variant as u8 != 0 }
}
/// Field `FIRE` reader - Joystick 2 fire (also keyboard column 4). Active low.
pub type FireR = crate::BitReader<Button>;
impl FireR {
    /// Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Button {
        match self.bits {
            false => Button::Pressed,
            true => Button::Released,
        }
    }

    /// `0`
    #[inline(always)]
    pub fn is_pressed(&self) -> bool { *self == Button::Pressed }

    /// `1`
    #[inline(always)]
    pub fn is_released(&self) -> bool { *self == Button::Released }
}
/// Field `FIRE` writer - Joystick 2 fire (also keyboard column 4). Active low.
pub type FireW<'a, REG> = crate::BitWriter<'a, REG, Button>;
impl<'a, REG> FireW<'a, REG> where REG: crate::Writable + crate::RegisterSpec, {
    /// `0`
    #[inline(always)]
    pub fn pressed(self) -> &'a mut crate::W<REG> { self.variant(Button::Pressed) }

    /// `1`
    #[inline(always)]
    pub fn released(self) -> &'a mut crate::W<REG> { self.variant(Button::Released) }
}
/// Field `PA5` reader - Keyboard column 5
pub type Pa5R = crate::BitReader;
/// Field `PA5` writer - Keyboard column 5
pub type Pa5W<'a, REG> = crate::BitWriter<'a, REG>;
/// Selects which control port's paddles are digitized by SID POTX/POTY: 1 =
/// port 1, 2 = port 2
///
/// Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Paddles {
    /// 1: `1`
    Port1 = 1,
    /// 2: `10`
    Port2 = 2,
}
impl From<Paddles> for u8 {
    #[inline(always)]
    fn from(variant: Paddles) -> Self { variant as _ }
}
impl crate::FieldSpec for Paddles {
    type Ux = u8;
}
impl crate::IsEnum for Paddles {}
/// Field `PADDLES` reader - Selects which control port's paddles are digitized
/// by SID POTX/POTY: 1 = port 1, 2 = port 2
pub type PaddlesR = crate::FieldReader<Paddles>;
impl PaddlesR {
    /// Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<Paddles> {
        match self.bits {
            1 => Some(Paddles::Port1),
            2 => Some(Paddles::Port2),
            _ => None,
        }
    }

    /// `1`
    #[inline(always)]
    pub fn is_port1(&self) -> bool { *self == Paddles::Port1 }

    /// `10`
    #[inline(always)]
    pub fn is_port2(&self) -> bool { *self == Paddles::Port2 }
}
/// Field `PADDLES` writer - Selects which control port's paddles are digitized
/// by SID POTX/POTY: 1 = port 1, 2 = port 2
pub type PaddlesW<'a, REG> = crate::FieldWriter<'a, REG, 2, Paddles>;
impl<'a, REG> PaddlesW<'a, REG>
    where REG: crate::Writable + crate::RegisterSpec,
          REG::Ux: From<u8>,
{
    /// `1`
    #[inline(always)]
    pub fn port1(self) -> &'a mut crate::W<REG> { self.variant(Paddles::Port1) }

    /// `10`
    #[inline(always)]
    pub fn port2(self) -> &'a mut crate::W<REG> { self.variant(Paddles::Port2) }
}
impl R {
    /// Bit 0 - Joystick 2 up (also keyboard column 0). Active low.
    #[inline(always)]
    pub fn up(&self) -> UpR { UpR::new((self.bits & 1) != 0) }

    /// Bit 1 - Joystick 2 down (also keyboard column 1). Active low.
    #[inline(always)]
    pub fn down(&self) -> DownR { DownR::new(((self.bits >> 1) & 1) != 0) }

    /// Bit 2 - Joystick 2 left (also keyboard column 2). Active low.
    #[inline(always)]
    pub fn left(&self) -> LeftR { LeftR::new(((self.bits >> 2) & 1) != 0) }

    /// Bit 3 - Joystick 2 right (also keyboard column 3). Active low.
    #[inline(always)]
    pub fn right(&self) -> RightR { RightR::new(((self.bits >> 3) & 1) != 0) }

    /// Bit 4 - Joystick 2 fire (also keyboard column 4). Active low.
    #[inline(always)]
    pub fn fire(&self) -> FireR { FireR::new(((self.bits >> 4) & 1) != 0) }

    /// Bit 5 - Keyboard column 5
    #[inline(always)]
    pub fn pa5(&self) -> Pa5R { Pa5R::new(((self.bits >> 5) & 1) != 0) }

    /// Bits 6:7 - Selects which control port's paddles are digitized by SID
    /// POTX/POTY: 1 = port 1, 2 = port 2
    #[inline(always)]
    pub fn paddles(&self) -> PaddlesR { PaddlesR::new((self.bits >> 6) & 3) }
}
impl W {
    /// Bit 0 - Joystick 2 up (also keyboard column 0). Active low.
    #[inline(always)]
    pub fn up(&mut self) -> UpW<'_, CiapraSpec> { UpW::new(self, 0) }

    /// Bit 1 - Joystick 2 down (also keyboard column 1). Active low.
    #[inline(always)]
    pub fn down(&mut self) -> DownW<'_, CiapraSpec> { DownW::new(self, 1) }

    /// Bit 2 - Joystick 2 left (also keyboard column 2). Active low.
    #[inline(always)]
    pub fn left(&mut self) -> LeftW<'_, CiapraSpec> { LeftW::new(self, 2) }

    /// Bit 3 - Joystick 2 right (also keyboard column 3). Active low.
    #[inline(always)]
    pub fn right(&mut self) -> RightW<'_, CiapraSpec> { RightW::new(self, 3) }

    /// Bit 4 - Joystick 2 fire (also keyboard column 4). Active low.
    #[inline(always)]
    pub fn fire(&mut self) -> FireW<'_, CiapraSpec> { FireW::new(self, 4) }

    /// Bit 5 - Keyboard column 5
    #[inline(always)]
    pub fn pa5(&mut self) -> Pa5W<'_, CiapraSpec> { Pa5W::new(self, 5) }

    /// Bits 6:7 - Selects which control port's paddles are digitized by SID
    /// POTX/POTY: 1 = port 1, 2 = port 2
    #[inline(always)]
    pub fn paddles(&mut self) -> PaddlesW<'_, CiapraSpec> { PaddlesW::new(self, 6) }
}
/// Port A. Doubles as keyboard-matrix column drive (outputs, active low) and
/// joystick port 2 (bits 0-4, active low). Bits 6-7 select which control port's
/// paddles reach SID POTX/POTY.
///
/// You can [`read`](crate::Reg::read) this register and get [`ciapra::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ciapra::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct CiapraSpec;
impl crate::RegisterSpec for CiapraSpec {
    type Ux = u8;
}
/// `read()` method returns [`ciapra::R`](R) reader structure
impl crate::Readable for CiapraSpec {}
/// `write(|w| ..)` method takes [`ciapra::W`](W) writer structure
impl crate::Writable for CiapraSpec {
    type Safety = crate::Unsafe;
}
/// `reset()` method sets CIAPRA to value 0
impl crate::Resettable for CiapraSpec {}
