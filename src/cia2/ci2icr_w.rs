/// Register `CI2ICR_W` writer
pub type W = crate::W<Ci2icrWSpec>;
/// Timer A interrupt mask select. `set()` marks Timer A for change; the
/// direction (enable vs disable) comes from `mode()`. `clear()` (default)
/// leaves Timer A untouched. Setting this alone does nothing without `mode()`.
pub use crate::cia1::ciaicr_w::SourceSelect;
/// Field `TIMER_A` writer - Timer A interrupt mask select. `set()` marks Timer
/// A for change; the direction (enable vs disable) comes from `mode()`.
/// `clear()` (default) leaves Timer A untouched. Setting this alone does
/// nothing without `mode()`.
pub use crate::cia1::ciaicr_w::TimerAW;
/// Field `TIMER_B` writer - Timer B interrupt mask select. `set()` marks Timer
/// B for change; the direction (enable vs disable) comes from `mode()`.
/// `clear()` (default) leaves Timer B untouched. Setting this alone does
/// nothing without `mode()`.
pub use crate::cia1::ciaicr_w::TimerAW as TimerBW;
/// Field `ALARM` writer - TOD alarm interrupt mask select. `set()` marks the
/// alarm for change; the direction (enable vs disable) comes from `mode()`.
/// `clear()` (default) leaves it untouched. Setting this alone does nothing
/// without `mode()`.
pub use crate::cia1::ciaicr_w::TimerAW as AlarmW;
/// Field `SERIAL` writer - Serial interrupt mask select. `set()` marks the
/// serial source for change; the direction (enable vs disable) comes from
/// `mode()`. `clear()` (default) leaves it untouched. Setting this alone does
/// nothing without `mode()`.
pub use crate::cia1::ciaicr_w::TimerAW as SerialW;
/// Field `FLAG` writer - FLAG interrupt mask select. `set()` marks the FLAG
/// source for change; the direction (enable vs disable) comes from `mode()`.
/// `clear()` (default) leaves it untouched. Setting this alone does nothing
/// without `mode()`.
pub use crate::cia1::ciaicr_w::TimerAW as FlagW;
/// Direction for this write: `enabled()` enables every source bit you `set()`;
/// `disabled()` disables them. Sources left `clear()` are untouched either way.
/// SIDE-EFFECT: the (un)mask takes effect immediately.
pub use crate::vic::scroly::Enable;
/// Field `MODE` writer - Direction for this write: `enabled()` enables every
/// source bit you `set()`; `disabled()` disables them. Sources left `clear()`
/// are untouched either way. SIDE-EFFECT: the (un)mask takes effect
/// immediately.
pub use crate::vic::scroly::ScreenW as ModeW;
impl core::fmt::Debug for crate::generic::Reg<Ci2icrWSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    /// Bit 0 - Timer A interrupt mask select. `set()` marks Timer A for change;
    /// the direction (enable vs disable) comes from `mode()`. `clear()`
    /// (default) leaves Timer A untouched. Setting this alone does nothing
    /// without `mode()`.
    #[inline(always)]
    pub fn timer_a(&mut self) -> TimerAW<'_, Ci2icrWSpec> { TimerAW::new(self, 0) }

    /// Bit 1 - Timer B interrupt mask select. `set()` marks Timer B for change;
    /// the direction (enable vs disable) comes from `mode()`. `clear()`
    /// (default) leaves Timer B untouched. Setting this alone does nothing
    /// without `mode()`.
    #[inline(always)]
    pub fn timer_b(&mut self) -> TimerBW<'_, Ci2icrWSpec> { TimerBW::new(self, 1) }

    /// Bit 2 - TOD alarm interrupt mask select. `set()` marks the alarm for
    /// change; the direction (enable vs disable) comes from `mode()`. `clear()`
    /// (default) leaves it untouched. Setting this alone does nothing without
    /// `mode()`.
    #[inline(always)]
    pub fn alarm(&mut self) -> AlarmW<'_, Ci2icrWSpec> { AlarmW::new(self, 2) }

    /// Bit 3 - Serial interrupt mask select. `set()` marks the serial source
    /// for change; the direction (enable vs disable) comes from `mode()`.
    /// `clear()` (default) leaves it untouched. Setting this alone does nothing
    /// without `mode()`.
    #[inline(always)]
    pub fn serial(&mut self) -> SerialW<'_, Ci2icrWSpec> { SerialW::new(self, 3) }

    /// Bit 4 - FLAG interrupt mask select. `set()` marks the FLAG source for
    /// change; the direction (enable vs disable) comes from `mode()`. `clear()`
    /// (default) leaves it untouched. Setting this alone does nothing without
    /// `mode()`.
    #[inline(always)]
    pub fn flag(&mut self) -> FlagW<'_, Ci2icrWSpec> { FlagW::new(self, 4) }

    /// Bit 7 - Direction for this write: `enabled()` enables every source bit
    /// you `set()`; `disabled()` disables them. Sources left `clear()` are
    /// untouched either way. SIDE-EFFECT: the (un)mask takes effect
    /// immediately.
    #[inline(always)]
    pub fn mode(&mut self) -> ModeW<'_, Ci2icrWSpec> { ModeW::new(self, 7) }
}
/// Interrupt mask (write-only alternate view of 0xDD0D). Enable or disable
/// individual NMI sources without disturbing the others: `set()` the sources
/// you want to change, then let `mode()` pick the direction —
/// `mode().enabled()` enables every source you `set()`, `mode().disabled()`
/// disables them. Sources left `clear()` (the default) are untouched either
/// way. Read status via CI2ICR_R. SIDE-EFFECT: this write changes which
/// interrupts are enabled.
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
