/// Register `CI2ICR_W` writer
pub type W = crate::W<Ci2icrWSpec>;
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
impl W {
    /// Bit 0 - Enable Timer A interrupt
    #[inline(always)]
    pub fn timer_a(&mut self) -> TimerAW<'_, Ci2icrWSpec> { TimerAW::new(self, 0) }

    /// Bit 1 - Enable Timer B interrupt
    #[inline(always)]
    pub fn timer_b(&mut self) -> TimerBW<'_, Ci2icrWSpec> { TimerBW::new(self, 1) }

    /// Bit 2 - Enable TOD alarm interrupt
    #[inline(always)]
    pub fn alarm(&mut self) -> AlarmW<'_, Ci2icrWSpec> { AlarmW::new(self, 2) }

    /// Bit 3 - Enable serial interrupt
    #[inline(always)]
    pub fn serial(&mut self) -> SerialW<'_, Ci2icrWSpec> { SerialW::new(self, 3) }

    /// Bit 4 - Enable FLAG interrupt
    #[inline(always)]
    pub fn flag(&mut self) -> FlagW<'_, Ci2icrWSpec> { FlagW::new(self, 4) }

    /// Bit 7 - Set/clear selector: enabled = the source bits you set are turned
    /// on, disabled = turned off; bits left 0 are unchanged. SIDE-EFFECT:
    /// (un)masking an interrupt takes effect at once.
    #[inline(always)]
    pub fn mode(&mut self) -> ModeW<'_, Ci2icrWSpec> { ModeW::new(self, 7) }
}
/// Interrupt mask (write-only alternate view of 0xDD0D). Bit 7 (MODE) is the
/// set/clear selector: with MODE enabled the source bits you enable are turned
/// on, with MODE disabled they are turned off; bits left 0 are unchanged. Read
/// status via CI2ICR_R. SIDE-EFFECT: this write changes which interrupts are
/// enabled.
///
/// You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ci2icr_w::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct Ci2icrWSpec;
impl crate::RegisterSpec for Ci2icrWSpec {
    type Ux = u8;
}
/// `write(|w| ..)` method takes [`ci2icr_w::W`](W) writer structure
impl crate::Writable for Ci2icrWSpec {
    type Safety = crate::Unsafe;
}
/// `reset()` method sets CI2ICR_W to value 0
impl crate::Resettable for Ci2icrWSpec {}
