/// Register `CIAICR_W` writer
pub type W = crate::W<CiaicrWSpec>;
/// Enable Timer A interrupt
pub use crate::vic::scroly::Enable;
/// Field `TIMER_A` writer - Enable Timer A interrupt
pub use crate::vic::scroly::ScreenW as TimerAW;
/// Field `TIMER_B` writer - Enable Timer B interrupt
pub use crate::vic::scroly::ScreenW as TimerBW;
/// Field `ALARM` writer - Enable TOD alarm interrupt
pub use crate::vic::scroly::ScreenW as AlarmW;
/// Field `SERIAL` writer - Enable serial interrupt
pub use crate::vic::scroly::ScreenW as SerialW;
/// Field `FLAG` writer - Enable FLAG interrupt
pub use crate::vic::scroly::ScreenW as FlagW;
/// Field `MODE` writer - Set/clear selector: enabled = the source bits you set
/// are turned on, disabled = turned off; bits left 0 are unchanged.
/// SIDE-EFFECT: (un)masking an interrupt takes effect at once.
pub use crate::vic::scroly::ScreenW as ModeW;
impl core::fmt::Debug for crate::generic::Reg<CiaicrWSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    /// Bit 0 - Enable Timer A interrupt
    #[inline(always)]
    pub fn timer_a(&mut self) -> TimerAW<'_, CiaicrWSpec> { TimerAW::new(self, 0) }

    /// Bit 1 - Enable Timer B interrupt
    #[inline(always)]
    pub fn timer_b(&mut self) -> TimerBW<'_, CiaicrWSpec> { TimerBW::new(self, 1) }

    /// Bit 2 - Enable TOD alarm interrupt
    #[inline(always)]
    pub fn alarm(&mut self) -> AlarmW<'_, CiaicrWSpec> { AlarmW::new(self, 2) }

    /// Bit 3 - Enable serial interrupt
    #[inline(always)]
    pub fn serial(&mut self) -> SerialW<'_, CiaicrWSpec> { SerialW::new(self, 3) }

    /// Bit 4 - Enable FLAG interrupt
    #[inline(always)]
    pub fn flag(&mut self) -> FlagW<'_, CiaicrWSpec> { FlagW::new(self, 4) }

    /// Bit 7 - Set/clear selector: enabled = the source bits you set are turned
    /// on, disabled = turned off; bits left 0 are unchanged. SIDE-EFFECT:
    /// (un)masking an interrupt takes effect at once.
    #[inline(always)]
    pub fn mode(&mut self) -> ModeW<'_, CiaicrWSpec> { ModeW::new(self, 7) }
}
/// Interrupt mask (write-only alternate view of 0xDC0D). Bit 7 (MODE) is the
/// set/clear selector: with MODE enabled the source bits you enable are turned
/// on, with MODE disabled they are turned off; bits left 0 are unchanged. Read
/// status via CIAICR_R. SIDE-EFFECT: this write changes which interrupts are
/// enabled.
///
/// You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ciaicr_w::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct CiaicrWSpec;
impl crate::RegisterSpec for CiaicrWSpec {
    type Ux = u8;
}
/// `write(|w| ..)` method takes [`ciaicr_w::W`](W) writer structure
impl crate::Writable for CiaicrWSpec {
    type Safety = crate::Unsafe;
}
/// `reset()` method sets CIAICR_W to value 0
impl crate::Resettable for CiaicrWSpec {}
