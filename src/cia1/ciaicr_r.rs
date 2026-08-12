/// Register `CIAICR_R` reader
pub type R = crate::R<CiaicrRSpec>;
/// Timer A underflow
///
/// Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Latch {
    /// 0: `0`
    Inactive = 0,
    /// 1: `1`
    Latched = 1,
}
impl From<Latch> for bool {
    #[inline(always)]
    fn from(variant: Latch) -> Self { variant as u8 != 0 }
}
/// Field `TIMER_A` reader - Timer A underflow
pub type TimerAR = crate::BitReader<Latch>;
impl TimerAR {
    /// Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Latch {
        match self.bits {
            false => Latch::Inactive,
            true => Latch::Latched,
        }
    }

    /// `0`
    #[inline(always)]
    pub fn is_inactive(&self) -> bool { *self == Latch::Inactive }

    /// `1`
    #[inline(always)]
    pub fn is_latched(&self) -> bool { *self == Latch::Latched }
}
/// Field `TIMER_B` reader - Timer B underflow
pub use TimerAR as TimerBR;
/// Field `ALARM` reader - TOD alarm match
pub use TimerAR as AlarmR;
/// Field `SERIAL` reader - Serial register full/empty
pub use TimerAR as SerialR;
/// Field `FLAG` reader - FLAG pin negative edge (CIA1: Datassette read)
pub use TimerAR as FlagR;
/// Field `IRQ` reader - Any enabled source latched (this read also clears all
/// flags)
pub use TimerAR as IrqR;
impl R {
    /// Bit 0 - Timer A underflow
    #[inline(always)]
    pub fn timer_a(&self) -> TimerAR { TimerAR::new((self.bits & 1) != 0) }

    /// Bit 1 - Timer B underflow
    #[inline(always)]
    pub fn timer_b(&self) -> TimerBR { TimerBR::new(((self.bits >> 1) & 1) != 0) }

    /// Bit 2 - TOD alarm match
    #[inline(always)]
    pub fn alarm(&self) -> AlarmR { AlarmR::new(((self.bits >> 2) & 1) != 0) }

    /// Bit 3 - Serial register full/empty
    #[inline(always)]
    pub fn serial(&self) -> SerialR { SerialR::new(((self.bits >> 3) & 1) != 0) }

    /// Bit 4 - FLAG pin negative edge (CIA1: Datassette read)
    #[inline(always)]
    pub fn flag(&self) -> FlagR { FlagR::new(((self.bits >> 4) & 1) != 0) }

    /// Bit 7 - Any enabled source latched (this read also clears all flags)
    #[inline(always)]
    pub fn irq(&self) -> IrqR { IrqR::new(((self.bits >> 7) & 1) != 0) }
}
/// Interrupt data (read-only view of 0xDC0D, asserts IRQ): which sources have
/// fired. Bit 7 (IR) reads Latched if any enabled source is pending.
/// SIDE-EFFECT: reading clears all flags and releases the line, so capture
/// everything from one read. Set the mask via CIAICR_W.
///
/// You can [`read`](crate::Reg::read) this register and get [`ciaicr_r::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
/// <div class="warning">The register is <b>cleared</b> (set to zero) following
/// a read operation.</div>
pub struct CiaicrRSpec;
impl crate::RegisterSpec for CiaicrRSpec {
    type Ux = u8;
}
/// `read()` method returns [`ciaicr_r::R`](R) reader structure
impl crate::Readable for CiaicrRSpec {}
/// `reset()` method sets CIAICR_R to value 0
impl crate::Resettable for CiaicrRSpec {}
