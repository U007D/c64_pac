/// Register `CI2CRB` reader
pub type R = crate::R<Ci2crbSpec>;
/// Register `CI2CRB` writer
pub type W = crate::W<Ci2crbSpec>;
/// Start/stop timer B. SIDE-EFFECT: writing Started begins the count
/// immediately.
///
/// Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StartTimer {
    /// 0: `0`
    Stopped = 0,
    /// 1: `1`
    Started = 1,
}
impl From<StartTimer> for bool {
    #[inline(always)]
    fn from(variant: StartTimer) -> Self { variant as u8 != 0 }
}
/// Field `START_TIMER` reader - Start/stop timer B. SIDE-EFFECT: writing
/// Started begins the count immediately.
pub type StartTimerR = crate::BitReader<StartTimer>;
impl StartTimerR {
    /// Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> StartTimer {
        match self.bits {
            false => StartTimer::Stopped,
            true => StartTimer::Started,
        }
    }

    /// `0`
    #[inline(always)]
    pub fn is_stopped(&self) -> bool { *self == StartTimer::Stopped }

    /// `1`
    #[inline(always)]
    pub fn is_started(&self) -> bool { *self == StartTimer::Started }
}
/// Field `START_TIMER` writer - Start/stop timer B. SIDE-EFFECT: writing
/// Started begins the count immediately.
pub type StartTimerW<'a, REG> = crate::BitWriter<'a, REG, StartTimer>;
impl<'a, REG> StartTimerW<'a, REG> where REG: crate::Writable + crate::RegisterSpec, {
    /// `0`
    #[inline(always)]
    pub fn stopped(self) -> &'a mut crate::W<REG> { self.variant(StartTimer::Stopped) }

    /// `1`
    #[inline(always)]
    pub fn started(self) -> &'a mut crate::W<REG> { self.variant(StartTimer::Started) }
}
/// Route timer B underflow onto PB7
///
/// Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SelectTimerOutput {
    /// 0: `0`
    OutputUnavailable = 0,
    /// 1: `1`
    OutputAvailable = 1,
}
impl From<SelectTimerOutput> for bool {
    #[inline(always)]
    fn from(variant: SelectTimerOutput) -> Self { variant as u8 != 0 }
}
/// Field `SELECT_TIMER_OUTPUT` reader - Route timer B underflow onto PB7
pub type SelectTimerOutputR = crate::BitReader<SelectTimerOutput>;
impl SelectTimerOutputR {
    /// Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SelectTimerOutput {
        match self.bits {
            false => SelectTimerOutput::OutputUnavailable,
            true => SelectTimerOutput::OutputAvailable,
        }
    }

    /// `0`
    #[inline(always)]
    pub fn is_output_unavailable(&self) -> bool { *self == SelectTimerOutput::OutputUnavailable }

    /// `1`
    #[inline(always)]
    pub fn is_output_available(&self) -> bool { *self == SelectTimerOutput::OutputAvailable }
}
/// Field `SELECT_TIMER_OUTPUT` writer - Route timer B underflow onto PB7
pub type SelectTimerOutputW<'a, REG> = crate::BitWriter<'a, REG, SelectTimerOutput>;
impl<'a, REG> SelectTimerOutputW<'a, REG> where REG: crate::Writable + crate::RegisterSpec, {
    /// `0`
    #[inline(always)]
    pub fn output_unavailable(self) -> &'a mut crate::W<REG> {
        self.variant(SelectTimerOutput::OutputUnavailable)
    }

    /// `1`
    #[inline(always)]
    pub fn output_available(self) -> &'a mut crate::W<REG> {
        self.variant(SelectTimerOutput::OutputAvailable)
    }
}
/// PB7 output waveform on timer B underflow
///
/// Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PortOutputMode {
    /// 0: `0`
    PulseBit7 = 0,
    /// 1: `1`
    ToggleBit7 = 1,
}
impl From<PortOutputMode> for bool {
    #[inline(always)]
    fn from(variant: PortOutputMode) -> Self { variant as u8 != 0 }
}
/// Field `PORT_OUTPUT_MODE` reader - PB7 output waveform on timer B underflow
pub type PortOutputModeR = crate::BitReader<PortOutputMode>;
impl PortOutputModeR {
    /// Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PortOutputMode {
        match self.bits {
            false => PortOutputMode::PulseBit7,
            true => PortOutputMode::ToggleBit7,
        }
    }

    /// `0`
    #[inline(always)]
    pub fn is_pulse_bit7(&self) -> bool { *self == PortOutputMode::PulseBit7 }

    /// `1`
    #[inline(always)]
    pub fn is_toggle_bit7(&self) -> bool { *self == PortOutputMode::ToggleBit7 }
}
/// Field `PORT_OUTPUT_MODE` writer - PB7 output waveform on timer B underflow
pub type PortOutputModeW<'a, REG> = crate::BitWriter<'a, REG, PortOutputMode>;
impl<'a, REG> PortOutputModeW<'a, REG> where REG: crate::Writable + crate::RegisterSpec, {
    /// `0`
    #[inline(always)]
    pub fn pulse_bit7(self) -> &'a mut crate::W<REG> { self.variant(PortOutputMode::PulseBit7) }

    /// `1`
    #[inline(always)]
    pub fn toggle_bit7(self) -> &'a mut crate::W<REG> { self.variant(PortOutputMode::ToggleBit7) }
}
/// Timer B reload behavior after underflow
///
/// Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TimerRunMode {
    /// 0: `0`
    Continuous = 0,
    /// 1: `1`
    OneShot = 1,
}
impl From<TimerRunMode> for bool {
    #[inline(always)]
    fn from(variant: TimerRunMode) -> Self { variant as u8 != 0 }
}
/// Field `TIMER_RUN_MODE` reader - Timer B reload behavior after underflow
pub type TimerRunModeR = crate::BitReader<TimerRunMode>;
impl TimerRunModeR {
    /// Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TimerRunMode {
        match self.bits {
            false => TimerRunMode::Continuous,
            true => TimerRunMode::OneShot,
        }
    }

    /// `0`
    #[inline(always)]
    pub fn is_continuous(&self) -> bool { *self == TimerRunMode::Continuous }

    /// `1`
    #[inline(always)]
    pub fn is_one_shot(&self) -> bool { *self == TimerRunMode::OneShot }
}
/// Field `TIMER_RUN_MODE` writer - Timer B reload behavior after underflow
pub type TimerRunModeW<'a, REG> = crate::BitWriter<'a, REG, TimerRunMode>;
impl<'a, REG> TimerRunModeW<'a, REG> where REG: crate::Writable + crate::RegisterSpec, {
    /// `0`
    #[inline(always)]
    pub fn continuous(self) -> &'a mut crate::W<REG> { self.variant(TimerRunMode::Continuous) }

    /// `1`
    #[inline(always)]
    pub fn one_shot(self) -> &'a mut crate::W<REG> { self.variant(TimerRunMode::OneShot) }
}
/// Force-load timer B from its latch (strobe, reads 0). SIDE-EFFECT: writing
/// Load copies the latch into the counter immediately.
///
/// Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ForceLatchedLoad {
    /// 0: `0`
    DoNothing = 0,
    /// 1: `1`
    Load = 1,
}
impl From<ForceLatchedLoad> for bool {
    #[inline(always)]
    fn from(variant: ForceLatchedLoad) -> Self { variant as u8 != 0 }
}
/// Field `FORCE_LATCHED_LOAD` reader - Force-load timer B from its latch
/// (strobe, reads 0). SIDE-EFFECT: writing Load copies the latch into the
/// counter immediately.
pub type ForceLatchedLoadR = crate::BitReader<ForceLatchedLoad>;
impl ForceLatchedLoadR {
    /// Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ForceLatchedLoad {
        match self.bits {
            false => ForceLatchedLoad::DoNothing,
            true => ForceLatchedLoad::Load,
        }
    }

    /// `0`
    #[inline(always)]
    pub fn is_do_nothing(&self) -> bool { *self == ForceLatchedLoad::DoNothing }

    /// `1`
    #[inline(always)]
    pub fn is_load(&self) -> bool { *self == ForceLatchedLoad::Load }
}
/// Field `FORCE_LATCHED_LOAD` writer - Force-load timer B from its latch
/// (strobe, reads 0). SIDE-EFFECT: writing Load copies the latch into the
/// counter immediately.
pub type ForceLatchedLoadW<'a, REG> = crate::BitWriter<'a, REG, ForceLatchedLoad>;
impl<'a, REG> ForceLatchedLoadW<'a, REG> where REG: crate::Writable + crate::RegisterSpec, {
    /// `0`
    #[inline(always)]
    pub fn do_nothing(self) -> &'a mut crate::W<REG> { self.variant(ForceLatchedLoad::DoNothing) }

    /// `1`
    #[inline(always)]
    pub fn load(self) -> &'a mut crate::W<REG> { self.variant(ForceLatchedLoad::Load) }
}
/// What timer B counts
///
/// Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TimerInputMode {
    /// 0: `0`
    CpuCycles = 0,
    /// 1: `1`
    UserPortCnt = 1,
    /// 2: `10`
    TaUnderflow = 2,
    /// 3: `11`
    TaUnderflowWhileCntHigh = 3,
}
impl From<TimerInputMode> for u8 {
    #[inline(always)]
    fn from(variant: TimerInputMode) -> Self { variant as _ }
}
impl crate::FieldSpec for TimerInputMode {
    type Ux = u8;
}
impl crate::IsEnum for TimerInputMode {}
/// Field `TIMER_INPUT_MODE` reader - What timer B counts
pub type TimerInputModeR = crate::FieldReader<TimerInputMode>;
impl TimerInputModeR {
    /// Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TimerInputMode {
        match self.bits {
            0 => TimerInputMode::CpuCycles,
            1 => TimerInputMode::UserPortCnt,
            2 => TimerInputMode::TaUnderflow,
            3 => TimerInputMode::TaUnderflowWhileCntHigh,
            _ => unreachable!(),
        }
    }

    /// `0`
    #[inline(always)]
    pub fn is_cpu_cycles(&self) -> bool { *self == TimerInputMode::CpuCycles }

    /// `1`
    #[inline(always)]
    pub fn is_user_port_cnt(&self) -> bool { *self == TimerInputMode::UserPortCnt }

    /// `10`
    #[inline(always)]
    pub fn is_ta_underflow(&self) -> bool { *self == TimerInputMode::TaUnderflow }

    /// `11`
    #[inline(always)]
    pub fn is_ta_underflow_while_cnt_high(&self) -> bool {
        *self == TimerInputMode::TaUnderflowWhileCntHigh
    }
}
/// Field `TIMER_INPUT_MODE` writer - What timer B counts
pub type TimerInputModeW<'a, REG> = crate::FieldWriter<'a, REG, 2, TimerInputMode, crate::Safe>;
impl<'a, REG> TimerInputModeW<'a, REG>
    where REG: crate::Writable + crate::RegisterSpec,
          REG::Ux: From<u8>,
{
    /// `0`
    #[inline(always)]
    pub fn cpu_cycles(self) -> &'a mut crate::W<REG> { self.variant(TimerInputMode::CpuCycles) }

    /// `1`
    #[inline(always)]
    pub fn user_port_cnt(self) -> &'a mut crate::W<REG> {
        self.variant(TimerInputMode::UserPortCnt)
    }

    /// `10`
    #[inline(always)]
    pub fn ta_underflow(self) -> &'a mut crate::W<REG> { self.variant(TimerInputMode::TaUnderflow) }

    /// `11`
    #[inline(always)]
    pub fn ta_underflow_while_cnt_high(self) -> &'a mut crate::W<REG> {
        self.variant(TimerInputMode::TaUnderflowWhileCntHigh)
    }
}
/// Selects what TOD register writes set: clock or alarm
///
/// Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TodWriteMode {
    /// 0: `0`
    Clock = 0,
    /// 1: `1`
    Alarm = 1,
}
impl From<TodWriteMode> for bool {
    #[inline(always)]
    fn from(variant: TodWriteMode) -> Self { variant as u8 != 0 }
}
/// Field `TOD_WRITE_MODE` reader - Selects what TOD register writes set: clock
/// or alarm
pub type TodWriteModeR = crate::BitReader<TodWriteMode>;
impl TodWriteModeR {
    /// Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TodWriteMode {
        match self.bits {
            false => TodWriteMode::Clock,
            true => TodWriteMode::Alarm,
        }
    }

    /// `0`
    #[inline(always)]
    pub fn is_clock(&self) -> bool { *self == TodWriteMode::Clock }

    /// `1`
    #[inline(always)]
    pub fn is_alarm(&self) -> bool { *self == TodWriteMode::Alarm }
}
/// Field `TOD_WRITE_MODE` writer - Selects what TOD register writes set: clock
/// or alarm
pub type TodWriteModeW<'a, REG> = crate::BitWriter<'a, REG, TodWriteMode>;
impl<'a, REG> TodWriteModeW<'a, REG> where REG: crate::Writable + crate::RegisterSpec, {
    /// `0`
    #[inline(always)]
    pub fn clock(self) -> &'a mut crate::W<REG> { self.variant(TodWriteMode::Clock) }

    /// `1`
    #[inline(always)]
    pub fn alarm(self) -> &'a mut crate::W<REG> { self.variant(TodWriteMode::Alarm) }
}
impl R {
    /// Bit 0 - Start/stop timer B. SIDE-EFFECT: writing Started begins the
    /// count immediately.
    #[inline(always)]
    pub fn start_timer(&self) -> StartTimerR { StartTimerR::new((self.bits & 1) != 0) }

    /// Bit 1 - Route timer B underflow onto PB7
    #[inline(always)]
    pub fn select_timer_output(&self) -> SelectTimerOutputR {
        SelectTimerOutputR::new(((self.bits >> 1) & 1) != 0)
    }

    /// Bit 2 - PB7 output waveform on timer B underflow
    #[inline(always)]
    pub fn port_output_mode(&self) -> PortOutputModeR {
        PortOutputModeR::new(((self.bits >> 2) & 1) != 0)
    }

    /// Bit 3 - Timer B reload behavior after underflow
    #[inline(always)]
    pub fn timer_run_mode(&self) -> TimerRunModeR {
        TimerRunModeR::new(((self.bits >> 3) & 1) != 0)
    }

    /// Bit 4 - Force-load timer B from its latch (strobe, reads 0).
    /// SIDE-EFFECT: writing Load copies the latch into the counter immediately.
    #[inline(always)]
    pub fn force_latched_load(&self) -> ForceLatchedLoadR {
        ForceLatchedLoadR::new(((self.bits >> 4) & 1) != 0)
    }

    /// Bits 5:6 - What timer B counts
    #[inline(always)]
    pub fn timer_input_mode(&self) -> TimerInputModeR { TimerInputModeR::new((self.bits >> 5) & 3) }

    /// Bit 7 - Selects what TOD register writes set: clock or alarm
    #[inline(always)]
    pub fn tod_write_mode(&self) -> TodWriteModeR {
        TodWriteModeR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    /// Bit 0 - Start/stop timer B. SIDE-EFFECT: writing Started begins the
    /// count immediately.
    #[inline(always)]
    pub fn start_timer(&mut self) -> StartTimerW<'_, Ci2crbSpec> { StartTimerW::new(self, 0) }

    /// Bit 1 - Route timer B underflow onto PB7
    #[inline(always)]
    pub fn select_timer_output(&mut self) -> SelectTimerOutputW<'_, Ci2crbSpec> {
        SelectTimerOutputW::new(self, 1)
    }

    /// Bit 2 - PB7 output waveform on timer B underflow
    #[inline(always)]
    pub fn port_output_mode(&mut self) -> PortOutputModeW<'_, Ci2crbSpec> {
        PortOutputModeW::new(self, 2)
    }

    /// Bit 3 - Timer B reload behavior after underflow
    #[inline(always)]
    pub fn timer_run_mode(&mut self) -> TimerRunModeW<'_, Ci2crbSpec> {
        TimerRunModeW::new(self, 3)
    }

    /// Bit 4 - Force-load timer B from its latch (strobe, reads 0).
    /// SIDE-EFFECT: writing Load copies the latch into the counter immediately.
    #[inline(always)]
    pub fn force_latched_load(&mut self) -> ForceLatchedLoadW<'_, Ci2crbSpec> {
        ForceLatchedLoadW::new(self, 4)
    }

    /// Bits 5:6 - What timer B counts
    #[inline(always)]
    pub fn timer_input_mode(&mut self) -> TimerInputModeW<'_, Ci2crbSpec> {
        TimerInputModeW::new(self, 5)
    }

    /// Bit 7 - Selects what TOD register writes set: clock or alarm
    #[inline(always)]
    pub fn tod_write_mode(&mut self) -> TodWriteModeW<'_, Ci2crbSpec> {
        TodWriteModeW::new(self, 7)
    }
}
/// Control register B (same layout as CIA1.CRB; the timer-output bits act on
/// this CIA's PB7)
///
/// You can [`read`](crate::Reg::read) this register and get [`ci2crb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ci2crb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct Ci2crbSpec;
impl crate::RegisterSpec for Ci2crbSpec {
    type Ux = u8;
}
/// `read()` method returns [`ci2crb::R`](R) reader structure
impl crate::Readable for Ci2crbSpec {}
/// `write(|w| ..)` method takes [`ci2crb::W`](W) writer structure
impl crate::Writable for Ci2crbSpec {
    type Safety = crate::Unsafe;
}
/// `reset()` method sets CI2CRB to value 0
impl crate::Resettable for Ci2crbSpec {}
