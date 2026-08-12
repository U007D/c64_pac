/// Register `CI2CRA` reader
pub type R = crate::R<Ci2craSpec>;
/// Register `CI2CRA` writer
pub type W = crate::W<Ci2craSpec>;
/// Start/stop timer A. SIDE-EFFECT: writing Started begins the count
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
/// Field `START_TIMER` reader - Start/stop timer A. SIDE-EFFECT: writing
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
/// Field `START_TIMER` writer - Start/stop timer A. SIDE-EFFECT: writing
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
/// Route timer A underflow onto PB6
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
/// Field `SELECT_TIMER_OUTPUT` reader - Route timer A underflow onto PB6
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
/// Field `SELECT_TIMER_OUTPUT` writer - Route timer A underflow onto PB6
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
/// PB6 output waveform on timer A underflow
///
/// Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PortOutputMode {
    /// 0: `0`
    PulseBit6 = 0,
    /// 1: `1`
    ToggleBit6 = 1,
}
impl From<PortOutputMode> for bool {
    #[inline(always)]
    fn from(variant: PortOutputMode) -> Self { variant as u8 != 0 }
}
/// Field `PORT_OUTPUT_MODE` reader - PB6 output waveform on timer A underflow
pub type PortOutputModeR = crate::BitReader<PortOutputMode>;
impl PortOutputModeR {
    /// Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PortOutputMode {
        match self.bits {
            false => PortOutputMode::PulseBit6,
            true => PortOutputMode::ToggleBit6,
        }
    }

    /// `0`
    #[inline(always)]
    pub fn is_pulse_bit6(&self) -> bool { *self == PortOutputMode::PulseBit6 }

    /// `1`
    #[inline(always)]
    pub fn is_toggle_bit6(&self) -> bool { *self == PortOutputMode::ToggleBit6 }
}
/// Field `PORT_OUTPUT_MODE` writer - PB6 output waveform on timer A underflow
pub type PortOutputModeW<'a, REG> = crate::BitWriter<'a, REG, PortOutputMode>;
impl<'a, REG> PortOutputModeW<'a, REG> where REG: crate::Writable + crate::RegisterSpec, {
    /// `0`
    #[inline(always)]
    pub fn pulse_bit6(self) -> &'a mut crate::W<REG> { self.variant(PortOutputMode::PulseBit6) }

    /// `1`
    #[inline(always)]
    pub fn toggle_bit6(self) -> &'a mut crate::W<REG> { self.variant(PortOutputMode::ToggleBit6) }
}
/// Timer A reload behavior after underflow
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
/// Field `TIMER_RUN_MODE` reader - Timer A reload behavior after underflow
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
/// Field `TIMER_RUN_MODE` writer - Timer A reload behavior after underflow
pub type TimerRunModeW<'a, REG> = crate::BitWriter<'a, REG, TimerRunMode>;
impl<'a, REG> TimerRunModeW<'a, REG> where REG: crate::Writable + crate::RegisterSpec, {
    /// `0`
    #[inline(always)]
    pub fn continuous(self) -> &'a mut crate::W<REG> { self.variant(TimerRunMode::Continuous) }

    /// `1`
    #[inline(always)]
    pub fn one_shot(self) -> &'a mut crate::W<REG> { self.variant(TimerRunMode::OneShot) }
}
/// Force-load timer A from its latch (strobe, reads 0). SIDE-EFFECT: writing
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
/// Field `FORCE_LATCHED_LOAD` reader - Force-load timer A from its latch
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
/// Field `FORCE_LATCHED_LOAD` writer - Force-load timer A from its latch
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
/// What timer A counts
///
/// Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TimerInputMode {
    /// 0: `0`
    CpuCycles = 0,
    /// 1: `1`
    UserPortCnt = 1,
}
impl From<TimerInputMode> for bool {
    #[inline(always)]
    fn from(variant: TimerInputMode) -> Self { variant as u8 != 0 }
}
/// Field `TIMER_INPUT_MODE` reader - What timer A counts
pub type TimerInputModeR = crate::BitReader<TimerInputMode>;
impl TimerInputModeR {
    /// Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TimerInputMode {
        match self.bits {
            false => TimerInputMode::CpuCycles,
            true => TimerInputMode::UserPortCnt,
        }
    }

    /// `0`
    #[inline(always)]
    pub fn is_cpu_cycles(&self) -> bool { *self == TimerInputMode::CpuCycles }

    /// `1`
    #[inline(always)]
    pub fn is_user_port_cnt(&self) -> bool { *self == TimerInputMode::UserPortCnt }
}
/// Field `TIMER_INPUT_MODE` writer - What timer A counts
pub type TimerInputModeW<'a, REG> = crate::BitWriter<'a, REG, TimerInputMode>;
impl<'a, REG> TimerInputModeW<'a, REG> where REG: crate::Writable + crate::RegisterSpec, {
    /// `0`
    #[inline(always)]
    pub fn cpu_cycles(self) -> &'a mut crate::W<REG> { self.variant(TimerInputMode::CpuCycles) }

    /// `1`
    #[inline(always)]
    pub fn user_port_cnt(self) -> &'a mut crate::W<REG> {
        self.variant(TimerInputMode::UserPortCnt)
    }
}
/// Serial-port shift-register direction
///
/// Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SerialPort {
    /// 0: `0`
    Input = 0,
    /// 1: `1`
    Output = 1,
}
impl From<SerialPort> for bool {
    #[inline(always)]
    fn from(variant: SerialPort) -> Self { variant as u8 != 0 }
}
/// Field `SERIAL_PORT` reader - Serial-port shift-register direction
pub type SerialPortR = crate::BitReader<SerialPort>;
impl SerialPortR {
    /// Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SerialPort {
        match self.bits {
            false => SerialPort::Input,
            true => SerialPort::Output,
        }
    }

    /// `0`
    #[inline(always)]
    pub fn is_input(&self) -> bool { *self == SerialPort::Input }

    /// `1`
    #[inline(always)]
    pub fn is_output(&self) -> bool { *self == SerialPort::Output }
}
/// Field `SERIAL_PORT` writer - Serial-port shift-register direction
pub type SerialPortW<'a, REG> = crate::BitWriter<'a, REG, SerialPort>;
impl<'a, REG> SerialPortW<'a, REG> where REG: crate::Writable + crate::RegisterSpec, {
    /// `0`
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> { self.variant(SerialPort::Input) }

    /// `1`
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> { self.variant(SerialPort::Output) }
}
/// TOD clock input frequency
///
/// Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TodClockFreq {
    /// 0: `0`
    Hz60 = 0,
    /// 1: `1`
    Hz50 = 1,
}
impl From<TodClockFreq> for bool {
    #[inline(always)]
    fn from(variant: TodClockFreq) -> Self { variant as u8 != 0 }
}
/// Field `TOD_CLOCK_FREQ` reader - TOD clock input frequency
pub type TodClockFreqR = crate::BitReader<TodClockFreq>;
impl TodClockFreqR {
    /// Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TodClockFreq {
        match self.bits {
            false => TodClockFreq::Hz60,
            true => TodClockFreq::Hz50,
        }
    }

    /// `0`
    #[inline(always)]
    pub fn is_hz60(&self) -> bool { *self == TodClockFreq::Hz60 }

    /// `1`
    #[inline(always)]
    pub fn is_hz50(&self) -> bool { *self == TodClockFreq::Hz50 }
}
/// Field `TOD_CLOCK_FREQ` writer - TOD clock input frequency
pub type TodClockFreqW<'a, REG> = crate::BitWriter<'a, REG, TodClockFreq>;
impl<'a, REG> TodClockFreqW<'a, REG> where REG: crate::Writable + crate::RegisterSpec, {
    /// `0`
    #[inline(always)]
    pub fn hz60(self) -> &'a mut crate::W<REG> { self.variant(TodClockFreq::Hz60) }

    /// `1`
    #[inline(always)]
    pub fn hz50(self) -> &'a mut crate::W<REG> { self.variant(TodClockFreq::Hz50) }
}
impl R {
    /// Bit 0 - Start/stop timer A. SIDE-EFFECT: writing Started begins the
    /// count immediately.
    #[inline(always)]
    pub fn start_timer(&self) -> StartTimerR { StartTimerR::new((self.bits & 1) != 0) }

    /// Bit 1 - Route timer A underflow onto PB6
    #[inline(always)]
    pub fn select_timer_output(&self) -> SelectTimerOutputR {
        SelectTimerOutputR::new(((self.bits >> 1) & 1) != 0)
    }

    /// Bit 2 - PB6 output waveform on timer A underflow
    #[inline(always)]
    pub fn port_output_mode(&self) -> PortOutputModeR {
        PortOutputModeR::new(((self.bits >> 2) & 1) != 0)
    }

    /// Bit 3 - Timer A reload behavior after underflow
    #[inline(always)]
    pub fn timer_run_mode(&self) -> TimerRunModeR {
        TimerRunModeR::new(((self.bits >> 3) & 1) != 0)
    }

    /// Bit 4 - Force-load timer A from its latch (strobe, reads 0).
    /// SIDE-EFFECT: writing Load copies the latch into the counter immediately.
    #[inline(always)]
    pub fn force_latched_load(&self) -> ForceLatchedLoadR {
        ForceLatchedLoadR::new(((self.bits >> 4) & 1) != 0)
    }

    /// Bit 5 - What timer A counts
    #[inline(always)]
    pub fn timer_input_mode(&self) -> TimerInputModeR {
        TimerInputModeR::new(((self.bits >> 5) & 1) != 0)
    }

    /// Bit 6 - Serial-port shift-register direction
    #[inline(always)]
    pub fn serial_port(&self) -> SerialPortR { SerialPortR::new(((self.bits >> 6) & 1) != 0) }

    /// Bit 7 - TOD clock input frequency
    #[inline(always)]
    pub fn tod_clock_freq(&self) -> TodClockFreqR {
        TodClockFreqR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CI2CRA")
         .field("start_timer", &self.start_timer())
         .field("select_timer_output", &self.select_timer_output())
         .field("port_output_mode", &self.port_output_mode())
         .field("timer_run_mode", &self.timer_run_mode())
         .field("force_latched_load", &self.force_latched_load())
         .field("timer_input_mode", &self.timer_input_mode())
         .field("serial_port", &self.serial_port())
         .field("tod_clock_freq", &self.tod_clock_freq())
         .finish()
    }
}
impl W {
    /// Bit 0 - Start/stop timer A. SIDE-EFFECT: writing Started begins the
    /// count immediately.
    #[inline(always)]
    pub fn start_timer(&mut self) -> StartTimerW<'_, Ci2craSpec> { StartTimerW::new(self, 0) }

    /// Bit 1 - Route timer A underflow onto PB6
    #[inline(always)]
    pub fn select_timer_output(&mut self) -> SelectTimerOutputW<'_, Ci2craSpec> {
        SelectTimerOutputW::new(self, 1)
    }

    /// Bit 2 - PB6 output waveform on timer A underflow
    #[inline(always)]
    pub fn port_output_mode(&mut self) -> PortOutputModeW<'_, Ci2craSpec> {
        PortOutputModeW::new(self, 2)
    }

    /// Bit 3 - Timer A reload behavior after underflow
    #[inline(always)]
    pub fn timer_run_mode(&mut self) -> TimerRunModeW<'_, Ci2craSpec> {
        TimerRunModeW::new(self, 3)
    }

    /// Bit 4 - Force-load timer A from its latch (strobe, reads 0).
    /// SIDE-EFFECT: writing Load copies the latch into the counter immediately.
    #[inline(always)]
    pub fn force_latched_load(&mut self) -> ForceLatchedLoadW<'_, Ci2craSpec> {
        ForceLatchedLoadW::new(self, 4)
    }

    /// Bit 5 - What timer A counts
    #[inline(always)]
    pub fn timer_input_mode(&mut self) -> TimerInputModeW<'_, Ci2craSpec> {
        TimerInputModeW::new(self, 5)
    }

    /// Bit 6 - Serial-port shift-register direction
    #[inline(always)]
    pub fn serial_port(&mut self) -> SerialPortW<'_, Ci2craSpec> { SerialPortW::new(self, 6) }

    /// Bit 7 - TOD clock input frequency
    #[inline(always)]
    pub fn tod_clock_freq(&mut self) -> TodClockFreqW<'_, Ci2craSpec> {
        TodClockFreqW::new(self, 7)
    }
}
/// Control register A (same layout as CIA1.CRA; the timer-output bits act on
/// this CIA's PB6)
///
/// You can [`read`](crate::Reg::read) this register and get [`ci2cra::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ci2cra::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct Ci2craSpec;
impl crate::RegisterSpec for Ci2craSpec {
    type Ux = u8;
}
/// `read()` method returns [`ci2cra::R`](R) reader structure
impl crate::Readable for Ci2craSpec {}
/// `write(|w| ..)` method takes [`ci2cra::W`](W) writer structure
impl crate::Writable for Ci2craSpec {
    type Safety = crate::Unsafe;
}
/// `reset()` method sets CI2CRA to value 0
impl crate::Resettable for Ci2craSpec {}
