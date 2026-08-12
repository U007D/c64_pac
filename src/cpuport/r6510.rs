/// Register `R6510` reader
pub type R = crate::R<R6510Spec>;
/// Register `R6510` writer
pub type W = crate::W<R6510Spec>;
/// 1 = BASIC ROM at 0xA000-0xBFFF (with HIRAM=1), 0 = RAM
///
/// Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Loram {
    /// 0: `0`
    Ram = 0,
    /// 1: `1`
    BasicRom = 1,
}
impl From<Loram> for bool {
    #[inline(always)]
    fn from(variant: Loram) -> Self { variant as u8 != 0 }
}
/// Field `LORAM` reader - 1 = BASIC ROM at 0xA000-0xBFFF (with HIRAM=1), 0 =
/// RAM
pub type LoramR = crate::BitReader<Loram>;
impl LoramR {
    /// Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Loram {
        match self.bits {
            false => Loram::Ram,
            true => Loram::BasicRom,
        }
    }

    /// `0`
    #[inline(always)]
    pub fn is_ram(&self) -> bool { *self == Loram::Ram }

    /// `1`
    #[inline(always)]
    pub fn is_basic_rom(&self) -> bool { *self == Loram::BasicRom }
}
/// Field `LORAM` writer - 1 = BASIC ROM at 0xA000-0xBFFF (with HIRAM=1), 0 =
/// RAM
pub type LoramW<'a, REG> = crate::BitWriter<'a, REG, Loram>;
impl<'a, REG> LoramW<'a, REG> where REG: crate::Writable + crate::RegisterSpec, {
    /// `0`
    #[inline(always)]
    pub fn ram(self) -> &'a mut crate::W<REG> { self.variant(Loram::Ram) }

    /// `1`
    #[inline(always)]
    pub fn basic_rom(self) -> &'a mut crate::W<REG> { self.variant(Loram::BasicRom) }
}
/// 1 = KERNAL ROM at 0xE000-0xFFFF, 0 = RAM
///
/// Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hiram {
    /// 0: `0`
    Ram = 0,
    /// 1: `1`
    KernalRom = 1,
}
impl From<Hiram> for bool {
    #[inline(always)]
    fn from(variant: Hiram) -> Self { variant as u8 != 0 }
}
/// Field `HIRAM` reader - 1 = KERNAL ROM at 0xE000-0xFFFF, 0 = RAM
pub type HiramR = crate::BitReader<Hiram>;
impl HiramR {
    /// Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Hiram {
        match self.bits {
            false => Hiram::Ram,
            true => Hiram::KernalRom,
        }
    }

    /// `0`
    #[inline(always)]
    pub fn is_ram(&self) -> bool { *self == Hiram::Ram }

    /// `1`
    #[inline(always)]
    pub fn is_kernal_rom(&self) -> bool { *self == Hiram::KernalRom }
}
/// Field `HIRAM` writer - 1 = KERNAL ROM at 0xE000-0xFFFF, 0 = RAM
pub type HiramW<'a, REG> = crate::BitWriter<'a, REG, Hiram>;
impl<'a, REG> HiramW<'a, REG> where REG: crate::Writable + crate::RegisterSpec, {
    /// `0`
    #[inline(always)]
    pub fn ram(self) -> &'a mut crate::W<REG> { self.variant(Hiram::Ram) }

    /// `1`
    #[inline(always)]
    pub fn kernal_rom(self) -> &'a mut crate::W<REG> { self.variant(Hiram::KernalRom) }
}
/// 1 = I/O at 0xD000-0xDFFF, 0 = character ROM (RAM if LORAM=HIRAM=0)
///
/// Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Charen {
    /// 0: `0`
    CharRom = 0,
    /// 1: `1`
    Io = 1,
}
impl From<Charen> for bool {
    #[inline(always)]
    fn from(variant: Charen) -> Self { variant as u8 != 0 }
}
/// Field `CHAREN` reader - 1 = I/O at 0xD000-0xDFFF, 0 = character ROM (RAM if
/// LORAM=HIRAM=0)
pub type CharenR = crate::BitReader<Charen>;
impl CharenR {
    /// Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Charen {
        match self.bits {
            false => Charen::CharRom,
            true => Charen::Io,
        }
    }

    /// `0`
    #[inline(always)]
    pub fn is_char_rom(&self) -> bool { *self == Charen::CharRom }

    /// `1`
    #[inline(always)]
    pub fn is_io(&self) -> bool { *self == Charen::Io }
}
/// Field `CHAREN` writer - 1 = I/O at 0xD000-0xDFFF, 0 = character ROM (RAM if
/// LORAM=HIRAM=0)
pub type CharenW<'a, REG> = crate::BitWriter<'a, REG, Charen>;
impl<'a, REG> CharenW<'a, REG> where REG: crate::Writable + crate::RegisterSpec, {
    /// `0`
    #[inline(always)]
    pub fn char_rom(self) -> &'a mut crate::W<REG> { self.variant(Charen::CharRom) }

    /// `1`
    #[inline(always)]
    pub fn io(self) -> &'a mut crate::W<REG> { self.variant(Charen::Io) }
}
/// Field `CASS_WRITE` reader - Datassette write line
pub type CassWriteR = crate::BitReader;
/// Field `CASS_WRITE` writer - Datassette write line
pub type CassWriteW<'a, REG> = crate::BitWriter<'a, REG>;
/// Datassette sense (input): 0 = a button is pressed
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
/// Field `CASS_SENSE` reader - Datassette sense (input): 0 = a button is
/// pressed
pub type CassSenseR = crate::BitReader<Button>;
impl CassSenseR {
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
/// Field `CASS_SENSE` writer - Datassette sense (input): 0 = a button is
/// pressed
pub type CassSenseW<'a, REG> = crate::BitWriter<'a, REG, Button>;
impl<'a, REG> CassSenseW<'a, REG> where REG: crate::Writable + crate::RegisterSpec, {
    /// `0`
    #[inline(always)]
    pub fn pressed(self) -> &'a mut crate::W<REG> { self.variant(Button::Pressed) }

    /// `1`
    #[inline(always)]
    pub fn released(self) -> &'a mut crate::W<REG> { self.variant(Button::Released) }
}
/// Datassette motor: 0 = on, 1 = off
///
/// Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CassMotor {
    /// 0: `0`
    Running = 0,
    /// 1: `1`
    Stopped = 1,
}
impl From<CassMotor> for bool {
    #[inline(always)]
    fn from(variant: CassMotor) -> Self { variant as u8 != 0 }
}
/// Field `CASS_MOTOR` reader - Datassette motor: 0 = on, 1 = off
pub type CassMotorR = crate::BitReader<CassMotor>;
impl CassMotorR {
    /// Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CassMotor {
        match self.bits {
            false => CassMotor::Running,
            true => CassMotor::Stopped,
        }
    }

    /// `0`
    #[inline(always)]
    pub fn is_running(&self) -> bool { *self == CassMotor::Running }

    /// `1`
    #[inline(always)]
    pub fn is_stopped(&self) -> bool { *self == CassMotor::Stopped }
}
/// Field `CASS_MOTOR` writer - Datassette motor: 0 = on, 1 = off
pub type CassMotorW<'a, REG> = crate::BitWriter<'a, REG, CassMotor>;
impl<'a, REG> CassMotorW<'a, REG> where REG: crate::Writable + crate::RegisterSpec, {
    /// `0`
    #[inline(always)]
    pub fn running(self) -> &'a mut crate::W<REG> { self.variant(CassMotor::Running) }

    /// `1`
    #[inline(always)]
    pub fn stopped(self) -> &'a mut crate::W<REG> { self.variant(CassMotor::Stopped) }
}
impl R {
    /// Bit 0 - 1 = BASIC ROM at 0xA000-0xBFFF (with HIRAM=1), 0 = RAM
    #[inline(always)]
    pub fn loram(&self) -> LoramR { LoramR::new((self.bits & 1) != 0) }

    /// Bit 1 - 1 = KERNAL ROM at 0xE000-0xFFFF, 0 = RAM
    #[inline(always)]
    pub fn hiram(&self) -> HiramR { HiramR::new(((self.bits >> 1) & 1) != 0) }

    /// Bit 2 - 1 = I/O at 0xD000-0xDFFF, 0 = character ROM (RAM if
    /// LORAM=HIRAM=0)
    #[inline(always)]
    pub fn charen(&self) -> CharenR { CharenR::new(((self.bits >> 2) & 1) != 0) }

    /// Bit 3 - Datassette write line
    #[inline(always)]
    pub fn cass_write(&self) -> CassWriteR { CassWriteR::new(((self.bits >> 3) & 1) != 0) }

    /// Bit 4 - Datassette sense (input): 0 = a button is pressed
    #[inline(always)]
    pub fn cass_sense(&self) -> CassSenseR { CassSenseR::new(((self.bits >> 4) & 1) != 0) }

    /// Bit 5 - Datassette motor: 0 = on, 1 = off
    #[inline(always)]
    pub fn cass_motor(&self) -> CassMotorR { CassMotorR::new(((self.bits >> 5) & 1) != 0) }
}
impl W {
    /// Bit 0 - 1 = BASIC ROM at 0xA000-0xBFFF (with HIRAM=1), 0 = RAM
    #[inline(always)]
    pub fn loram(&mut self) -> LoramW<'_, R6510Spec> { LoramW::new(self, 0) }

    /// Bit 1 - 1 = KERNAL ROM at 0xE000-0xFFFF, 0 = RAM
    #[inline(always)]
    pub fn hiram(&mut self) -> HiramW<'_, R6510Spec> { HiramW::new(self, 1) }

    /// Bit 2 - 1 = I/O at 0xD000-0xDFFF, 0 = character ROM (RAM if
    /// LORAM=HIRAM=0)
    #[inline(always)]
    pub fn charen(&mut self) -> CharenW<'_, R6510Spec> { CharenW::new(self, 2) }

    /// Bit 3 - Datassette write line
    #[inline(always)]
    pub fn cass_write(&mut self) -> CassWriteW<'_, R6510Spec> { CassWriteW::new(self, 3) }

    /// Bit 4 - Datassette sense (input): 0 = a button is pressed
    #[inline(always)]
    pub fn cass_sense(&mut self) -> CassSenseW<'_, R6510Spec> { CassSenseW::new(self, 4) }

    /// Bit 5 - Datassette motor: 0 = on, 1 = off
    #[inline(always)]
    pub fn cass_motor(&mut self) -> CassMotorW<'_, R6510Spec> { CassMotorW::new(self, 5) }
}
/// Banking and Datassette port. Reads return pin state for input bits.
///
/// You can [`read`](crate::Reg::read) this register and get [`r6510::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r6510::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct R6510Spec;
impl crate::RegisterSpec for R6510Spec {
    type Ux = u8;
}
/// `read()` method returns [`r6510::R`](R) reader structure
impl crate::Readable for R6510Spec {}
/// `write(|w| ..)` method takes [`r6510::W`](W) writer structure
impl crate::Writable for R6510Spec {
    type Safety = crate::Unsafe;
}
/// `reset()` method sets R6510 to value 0
impl crate::Resettable for R6510Spec {}
