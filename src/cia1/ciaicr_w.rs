/// Register `CIAICR_W` writer
pub type W = crate::W<CiaicrWSpec>;
/// Timer A interrupt mask select. `set()` marks Timer A for change; the
/// direction (enable vs disable) comes from `mode()`. `clear()` (default)
/// leaves Timer A untouched. Setting this alone does nothing without `mode()`.
///
/// Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SourceSelect {
    /// 0: Leave this source's mask unchanged
    Clear = 0,
    /// 1: Select this source for the update chosen by MODE
    Set = 1,
}
impl From<SourceSelect> for bool {
    #[inline(always)]
    fn from(variant: SourceSelect) -> Self { variant as u8 != 0 }
}
/// Field `TIMER_A` writer - Timer A interrupt mask select. `set()` marks Timer
/// A for change; the direction (enable vs disable) comes from `mode()`.
/// `clear()` (default) leaves Timer A untouched. Setting this alone does
/// nothing without `mode()`.
///
/// **Shared field type.** The same writer is reused for the fields below;
/// each keeps its own description — click through to read it in context:
/// - [`cia1::ciaicr_w::AlarmW`](crate::cia1::ciaicr_w::AlarmW)
/// - [`cia1::ciaicr_w::FlagW`](crate::cia1::ciaicr_w::FlagW)
/// - [`cia1::ciaicr_w::SerialW`](crate::cia1::ciaicr_w::SerialW)
/// - [`cia1::ciaicr_w::TimerBW`](crate::cia1::ciaicr_w::TimerBW)
/// - [`cia2::ci2icr_w::AlarmW`](crate::cia2::ci2icr_w::AlarmW)
/// - [`cia2::ci2icr_w::FlagW`](crate::cia2::ci2icr_w::FlagW)
/// - [`cia2::ci2icr_w::SerialW`](crate::cia2::ci2icr_w::SerialW)
/// - [`cia2::ci2icr_w::TimerBW`](crate::cia2::ci2icr_w::TimerBW)
pub type TimerAW<'a, REG> = crate::BitWriter<'a, REG, SourceSelect>;
impl<'a, REG> TimerAW<'a, REG> where REG: crate::Writable + crate::RegisterSpec, {
    /// Leave this source's mask unchanged
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> { self.variant(SourceSelect::Clear) }

    /// Select this source for the update chosen by MODE
    #[inline(always)]
    pub fn set(self) -> &'a mut crate::W<REG> { self.variant(SourceSelect::Set) }
}
/// Field `TIMER_B` writer - Timer B interrupt mask select. `set()` marks Timer
/// B for change; the direction (enable vs disable) comes from `mode()`.
/// `clear()` (default) leaves Timer B untouched. Setting this alone does
/// nothing without `mode()`.
pub use TimerAW as TimerBW;
/// Field `ALARM` writer - TOD alarm interrupt mask select. `set()` marks the
/// alarm for change; the direction (enable vs disable) comes from `mode()`.
/// `clear()` (default) leaves it untouched. Setting this alone does nothing
/// without `mode()`.
pub use TimerAW as AlarmW;
/// Field `SERIAL` writer - Serial interrupt mask select. `set()` marks the
/// serial source for change; the direction (enable vs disable) comes from
/// `mode()`. `clear()` (default) leaves it untouched. Setting this alone does
/// nothing without `mode()`.
pub use TimerAW as SerialW;
/// Field `FLAG` writer - FLAG interrupt mask select. `set()` marks the FLAG
/// source for change; the direction (enable vs disable) comes from `mode()`.
/// `clear()` (default) leaves it untouched. Setting this alone does nothing
/// without `mode()`.
pub use TimerAW as FlagW;

/// Direction for this write: `enabled()` enables every source bit you `set()`;
/// `disabled()` disables them. Sources left `clear()` are untouched either way.
/// SIDE-EFFECT: the (un)mask takes effect immediately.
pub use crate::vic::scroly::Enable;
/// Field `MODE` writer - Direction for this write: `enabled()` enables every
/// source bit you `set()`; `disabled()` disables them. Sources left `clear()`
/// are untouched either way. SIDE-EFFECT: the (un)mask takes effect
/// immediately.
pub use crate::vic::scroly::ScreenW as ModeW;
impl core::fmt::Debug for crate::generic::Reg<CiaicrWSpec> {
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
    pub fn timer_a(&mut self) -> TimerAW<'_, CiaicrWSpec> { TimerAW::new(self, 0) }

    /// Bit 1 - Timer B interrupt mask select. `set()` marks Timer B for change;
    /// the direction (enable vs disable) comes from `mode()`. `clear()`
    /// (default) leaves Timer B untouched. Setting this alone does nothing
    /// without `mode()`.
    #[inline(always)]
    pub fn timer_b(&mut self) -> TimerBW<'_, CiaicrWSpec> { TimerBW::new(self, 1) }

    /// Bit 2 - TOD alarm interrupt mask select. `set()` marks the alarm for
    /// change; the direction (enable vs disable) comes from `mode()`. `clear()`
    /// (default) leaves it untouched. Setting this alone does nothing without
    /// `mode()`.
    #[inline(always)]
    pub fn alarm(&mut self) -> AlarmW<'_, CiaicrWSpec> { AlarmW::new(self, 2) }

    /// Bit 3 - Serial interrupt mask select. `set()` marks the serial source
    /// for change; the direction (enable vs disable) comes from `mode()`.
    /// `clear()` (default) leaves it untouched. Setting this alone does nothing
    /// without `mode()`.
    #[inline(always)]
    pub fn serial(&mut self) -> SerialW<'_, CiaicrWSpec> { SerialW::new(self, 3) }

    /// Bit 4 - FLAG interrupt mask select. `set()` marks the FLAG source for
    /// change; the direction (enable vs disable) comes from `mode()`. `clear()`
    /// (default) leaves it untouched. Setting this alone does nothing without
    /// `mode()`.
    #[inline(always)]
    pub fn flag(&mut self) -> FlagW<'_, CiaicrWSpec> { FlagW::new(self, 4) }

    /// Bit 7 - Direction for this write: `enabled()` enables every source bit
    /// you `set()`; `disabled()` disables them. Sources left `clear()` are
    /// untouched either way. SIDE-EFFECT: the (un)mask takes effect
    /// immediately.
    #[inline(always)]
    pub fn mode(&mut self) -> ModeW<'_, CiaicrWSpec> { ModeW::new(self, 7) }
}
/// Interrupt mask (write-only alternate view of 0xDC0D). Enable or disable
/// individual interrupt sources without disturbing the others: `set()` the
/// sources you want to change, then let `mode()` pick the direction —
/// `mode().enabled()` enables every source you `set()`, `mode().disabled()`
/// disables them. Sources left `clear()` (the default) are untouched either
/// way. Read status via CIAICR_R. SIDE-EFFECT: this write changes which
/// interrupts are enabled.
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
