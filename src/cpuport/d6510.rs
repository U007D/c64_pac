/// Register `D6510` reader
pub type R = crate::R<D6510Spec>;
/// Register `D6510` writer
pub type W = crate::W<D6510Spec>;
/// Direction of the LORAM line (R6510 bit 0)
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
/// Field `LORAM` reader - Direction of the LORAM line (R6510 bit 0)
pub type LoramR = crate::BitReader<Direction>;
impl LoramR {
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
/// Field `LORAM` writer - Direction of the LORAM line (R6510 bit 0)
pub type LoramW<'a, REG> = crate::BitWriter<'a, REG, Direction>;
impl<'a, REG> LoramW<'a, REG> where REG: crate::Writable + crate::RegisterSpec, {
    /// `0`
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> { self.variant(Direction::Input) }

    /// `1`
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> { self.variant(Direction::Output) }
}
/// Field `HIRAM` reader - Direction of the HIRAM line (R6510 bit 1)
pub use LoramR as HiramR;
/// Field `CHAREN` reader - Direction of the CHAREN line (R6510 bit 2)
pub use LoramR as CharenR;
/// Field `CASS_WRITE` reader - Direction of the CASS_WRITE line (R6510 bit 3)
pub use LoramR as CassWriteR;
/// Field `CASS_SENSE` reader - Direction of the CASS_SENSE line (R6510 bit 4)
pub use LoramR as CassSenseR;
/// Field `CASS_MOTOR` reader - Direction of the CASS_MOTOR line (R6510 bit 5)
pub use LoramR as CassMotorR;
/// Field `HIRAM` writer - Direction of the HIRAM line (R6510 bit 1)
pub use LoramW as HiramW;
/// Field `CHAREN` writer - Direction of the CHAREN line (R6510 bit 2)
pub use LoramW as CharenW;
/// Field `CASS_WRITE` writer - Direction of the CASS_WRITE line (R6510 bit 3)
pub use LoramW as CassWriteW;
/// Field `CASS_SENSE` writer - Direction of the CASS_SENSE line (R6510 bit 4)
pub use LoramW as CassSenseW;
/// Field `CASS_MOTOR` writer - Direction of the CASS_MOTOR line (R6510 bit 5)
pub use LoramW as CassMotorW;
impl R {
    /// Bit 0 - Direction of the LORAM line (R6510 bit 0)
    #[inline(always)]
    pub fn loram(&self) -> LoramR { LoramR::new((self.bits & 1) != 0) }

    /// Bit 1 - Direction of the HIRAM line (R6510 bit 1)
    #[inline(always)]
    pub fn hiram(&self) -> HiramR { HiramR::new(((self.bits >> 1) & 1) != 0) }

    /// Bit 2 - Direction of the CHAREN line (R6510 bit 2)
    #[inline(always)]
    pub fn charen(&self) -> CharenR { CharenR::new(((self.bits >> 2) & 1) != 0) }

    /// Bit 3 - Direction of the CASS_WRITE line (R6510 bit 3)
    #[inline(always)]
    pub fn cass_write(&self) -> CassWriteR { CassWriteR::new(((self.bits >> 3) & 1) != 0) }

    /// Bit 4 - Direction of the CASS_SENSE line (R6510 bit 4)
    #[inline(always)]
    pub fn cass_sense(&self) -> CassSenseR { CassSenseR::new(((self.bits >> 4) & 1) != 0) }

    /// Bit 5 - Direction of the CASS_MOTOR line (R6510 bit 5)
    #[inline(always)]
    pub fn cass_motor(&self) -> CassMotorR { CassMotorR::new(((self.bits >> 5) & 1) != 0) }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("D6510")
         .field("loram", &self.loram())
         .field("hiram", &self.hiram())
         .field("charen", &self.charen())
         .field("cass_write", &self.cass_write())
         .field("cass_sense", &self.cass_sense())
         .field("cass_motor", &self.cass_motor())
         .finish()
    }
}
impl W {
    /// Bit 0 - Direction of the LORAM line (R6510 bit 0)
    #[inline(always)]
    pub fn loram(&mut self) -> LoramW<'_, D6510Spec> { LoramW::new(self, 0) }

    /// Bit 1 - Direction of the HIRAM line (R6510 bit 1)
    #[inline(always)]
    pub fn hiram(&mut self) -> HiramW<'_, D6510Spec> { HiramW::new(self, 1) }

    /// Bit 2 - Direction of the CHAREN line (R6510 bit 2)
    #[inline(always)]
    pub fn charen(&mut self) -> CharenW<'_, D6510Spec> { CharenW::new(self, 2) }

    /// Bit 3 - Direction of the CASS_WRITE line (R6510 bit 3)
    #[inline(always)]
    pub fn cass_write(&mut self) -> CassWriteW<'_, D6510Spec> { CassWriteW::new(self, 3) }

    /// Bit 4 - Direction of the CASS_SENSE line (R6510 bit 4)
    #[inline(always)]
    pub fn cass_sense(&mut self) -> CassSenseW<'_, D6510Spec> { CassSenseW::new(self, 4) }

    /// Bit 5 - Direction of the CASS_MOTOR line (R6510 bit 5)
    #[inline(always)]
    pub fn cass_motor(&mut self) -> CassMotorW<'_, D6510Spec> { CassMotorW::new(self, 5) }
}
/// Data-direction register for the 6510's on-chip I/O port (R6510): one bit per
/// line, 1 = output, 0 = input. The 6510 is a 6502 with this 6-bit port bolted
/// on; it is the whole reason the C64 can map 64 KiB RAM + 20 KiB ROM + 4 KiB
/// I/O (= 88 KiB (!)) into a single 64 KiB address space with no external MMU.
/// KERNAL reset writes 0x2F (0b0010_1111): lines 0-3 and 5 are outputs, line 4
/// (cassette sense) is an input, and bits 6-7 are unused (no port line).
/// Written before R6510 during reset. The RAM byte physically at 0x0000 still
/// exists but is hidden behind the port.
///
/// You can [`read`](crate::Reg::read) this register and get [`d6510::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d6510::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct D6510Spec;
impl crate::RegisterSpec for D6510Spec {
    type Ux = u8;
}
/// `read()` method returns [`d6510::R`](R) reader structure
impl crate::Readable for D6510Spec {}
/// `write(|w| ..)` method takes [`d6510::W`](W) writer structure
impl crate::Writable for D6510Spec {
    type Safety = crate::Unsafe;
}
/// `reset()` method sets D6510 to value 0
impl crate::Resettable for D6510Spec {}
