/// Register `CIAPRB` reader
pub type R = crate::R<CiaprbSpec>;
/// Register `CIAPRB` writer
pub type W = crate::W<CiaprbSpec>;
/// Joystick 1 up (also keyboard row 0). Active low.
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
/// Field `UP` reader - Joystick 1 up (also keyboard row 0). Active low.
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
/// Field `UP` writer - Joystick 1 up (also keyboard row 0). Active low.
pub type UpW<'a, REG> = crate::BitWriter<'a, REG, Direction>;
impl<'a, REG> UpW<'a, REG> where REG: crate::Writable + crate::RegisterSpec, {
    /// `0`
    #[inline(always)]
    pub fn pushed(self) -> &'a mut crate::W<REG> { self.variant(Direction::Pushed) }

    /// `1`
    #[inline(always)]
    pub fn released(self) -> &'a mut crate::W<REG> { self.variant(Direction::Released) }
}
/// Field `DOWN` reader - Joystick 1 down (also keyboard row 1). Active low.
pub use UpR as DownR;
/// Field `LEFT` reader - Joystick 1 left (also keyboard row 2). Active low.
pub use UpR as LeftR;
/// Field `RIGHT` reader - Joystick 1 right (also keyboard row 3). Active low.
pub use UpR as RightR;
/// Field `DOWN` writer - Joystick 1 down (also keyboard row 1). Active low.
pub use UpW as DownW;
/// Field `LEFT` writer - Joystick 1 left (also keyboard row 2). Active low.
pub use UpW as LeftW;
/// Field `RIGHT` writer - Joystick 1 right (also keyboard row 3). Active low.
pub use UpW as RightW;
/// Joystick 1 fire, also the light pen trigger (also keyboard row 4). Active
/// low.
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
/// Field `FIRE` reader - Joystick 1 fire, also the light pen trigger (also
/// keyboard row 4). Active low.
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
/// Field `FIRE` writer - Joystick 1 fire, also the light pen trigger (also
/// keyboard row 4). Active low.
pub type FireW<'a, REG> = crate::BitWriter<'a, REG, Button>;
impl<'a, REG> FireW<'a, REG> where REG: crate::Writable + crate::RegisterSpec, {
    /// `0`
    #[inline(always)]
    pub fn pressed(self) -> &'a mut crate::W<REG> { self.variant(Button::Pressed) }

    /// `1`
    #[inline(always)]
    pub fn released(self) -> &'a mut crate::W<REG> { self.variant(Button::Released) }
}
/// Field `PB5` reader - Keyboard row 5
pub type Pb5R = crate::BitReader;
/// Field `PB5` writer - Keyboard row 5
pub type Pb5W<'a, REG> = crate::BitWriter<'a, REG>;
/// Field `PB6` reader - Timer A underflow output when
/// CIACRA.SELECT_TIMER_OUTPUT is set; otherwise general I/O
pub type Pb6R = crate::BitReader;
/// Field `PB6` writer - Timer A underflow output when
/// CIACRA.SELECT_TIMER_OUTPUT is set; otherwise general I/O
pub type Pb6W<'a, REG> = crate::BitWriter<'a, REG>;
/// Field `PB7` reader - Timer B underflow output when
/// CIACRB.SELECT_TIMER_OUTPUT is set; otherwise general I/O
pub type Pb7R = crate::BitReader;
/// Field `PB7` writer - Timer B underflow output when
/// CIACRB.SELECT_TIMER_OUTPUT is set; otherwise general I/O
pub type Pb7W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    /// Bit 0 - Joystick 1 up (also keyboard row 0). Active low.
    #[inline(always)]
    pub fn up(&self) -> UpR { UpR::new((self.bits & 1) != 0) }

    /// Bit 1 - Joystick 1 down (also keyboard row 1). Active low.
    #[inline(always)]
    pub fn down(&self) -> DownR { DownR::new(((self.bits >> 1) & 1) != 0) }

    /// Bit 2 - Joystick 1 left (also keyboard row 2). Active low.
    #[inline(always)]
    pub fn left(&self) -> LeftR { LeftR::new(((self.bits >> 2) & 1) != 0) }

    /// Bit 3 - Joystick 1 right (also keyboard row 3). Active low.
    #[inline(always)]
    pub fn right(&self) -> RightR { RightR::new(((self.bits >> 3) & 1) != 0) }

    /// Bit 4 - Joystick 1 fire, also the light pen trigger (also keyboard row
    /// 4). Active low.
    #[inline(always)]
    pub fn fire(&self) -> FireR { FireR::new(((self.bits >> 4) & 1) != 0) }

    /// Bit 5 - Keyboard row 5
    #[inline(always)]
    pub fn pb5(&self) -> Pb5R { Pb5R::new(((self.bits >> 5) & 1) != 0) }

    /// Bit 6 - Timer A underflow output when CIACRA.SELECT_TIMER_OUTPUT is set;
    /// otherwise general I/O
    #[inline(always)]
    pub fn pb6(&self) -> Pb6R { Pb6R::new(((self.bits >> 6) & 1) != 0) }

    /// Bit 7 - Timer B underflow output when CIACRB.SELECT_TIMER_OUTPUT is set;
    /// otherwise general I/O
    #[inline(always)]
    pub fn pb7(&self) -> Pb7R { Pb7R::new(((self.bits >> 7) & 1) != 0) }
}
impl W {
    /// Bit 0 - Joystick 1 up (also keyboard row 0). Active low.
    #[inline(always)]
    pub fn up(&mut self) -> UpW<'_, CiaprbSpec> { UpW::new(self, 0) }

    /// Bit 1 - Joystick 1 down (also keyboard row 1). Active low.
    #[inline(always)]
    pub fn down(&mut self) -> DownW<'_, CiaprbSpec> { DownW::new(self, 1) }

    /// Bit 2 - Joystick 1 left (also keyboard row 2). Active low.
    #[inline(always)]
    pub fn left(&mut self) -> LeftW<'_, CiaprbSpec> { LeftW::new(self, 2) }

    /// Bit 3 - Joystick 1 right (also keyboard row 3). Active low.
    #[inline(always)]
    pub fn right(&mut self) -> RightW<'_, CiaprbSpec> { RightW::new(self, 3) }

    /// Bit 4 - Joystick 1 fire, also the light pen trigger (also keyboard row
    /// 4). Active low.
    #[inline(always)]
    pub fn fire(&mut self) -> FireW<'_, CiaprbSpec> { FireW::new(self, 4) }

    /// Bit 5 - Keyboard row 5
    #[inline(always)]
    pub fn pb5(&mut self) -> Pb5W<'_, CiaprbSpec> { Pb5W::new(self, 5) }

    /// Bit 6 - Timer A underflow output when CIACRA.SELECT_TIMER_OUTPUT is set;
    /// otherwise general I/O
    #[inline(always)]
    pub fn pb6(&mut self) -> Pb6W<'_, CiaprbSpec> { Pb6W::new(self, 6) }

    /// Bit 7 - Timer B underflow output when CIACRB.SELECT_TIMER_OUTPUT is set;
    /// otherwise general I/O
    #[inline(always)]
    pub fn pb7(&mut self) -> Pb7W<'_, CiaprbSpec> { Pb7W::new(self, 7) }
}
/// Port B. Doubles as keyboard-matrix row sense (inputs, active low) and
/// joystick port 1 (bits 0-4, active low; the fire line also triggers the light
/// pen). PB6/PB7 can output timer A/B underflow.
///
/// You can [`read`](crate::Reg::read) this register and get [`ciaprb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ciaprb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct CiaprbSpec;
impl crate::RegisterSpec for CiaprbSpec {
    type Ux = u8;
}
/// `read()` method returns [`ciaprb::R`](R) reader structure
impl crate::Readable for CiaprbSpec {}
/// `write(|w| ..)` method takes [`ciaprb::W`](W) writer structure
impl crate::Writable for CiaprbSpec {
    type Safety = crate::Unsafe;
}
/// `reset()` method sets CIAPRB to value 0
impl crate::Resettable for CiaprbSpec {}
