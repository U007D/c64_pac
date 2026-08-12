/// Register `CI2ICR_R` reader
pub type R = crate::R<Ci2icrRSpec>;
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
/// Field `FLAG` reader - FLAG2 negative edge (user port pin B / RS-232 RXD
/// start bit)
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

    /// Bit 4 - FLAG2 negative edge (user port pin B / RS-232 RXD start bit)
    #[inline(always)]
    pub fn flag(&self) -> FlagR { FlagR::new(((self.bits >> 4) & 1) != 0) }

    /// Bit 7 - Any enabled source latched (this read also clears all flags)
    #[inline(always)]
    pub fn irq(&self) -> IrqR { IrqR::new(((self.bits >> 7) & 1) != 0) }
}
impl core::fmt::Debug for crate::generic::Reg<Ci2icrRSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
/// Interrupt data (read-only view of 0xDD0D, asserts NMI): which sources have
/// fired. Bit 7 (IR) reads Latched if any enabled source is pending.
/// SIDE-EFFECT: reading clears all flags and releases the line, so capture
/// everything from one read. Set the mask via CI2ICR_W.
///
/// You can [`read`](crate::Reg::read) this register and get [`ci2icr_r::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
/// <div class="warning">The register is <b>cleared</b> (set to zero) following
/// a read operation.</div>
pub struct Ci2icrRSpec;
impl crate::RegisterSpec for Ci2icrRSpec {
    type Ux = u8;
}
/// `read()` method returns [`ci2icr_r::R`](R) reader structure
impl crate::Readable for Ci2icrRSpec {}
/// `reset()` method sets CI2ICR_R to value 0
impl crate::Resettable for Ci2icrRSpec {}
