/// Register `PWLO` writer
pub type W = crate::W<PwloSpec>;
/// Field `PULSE_WAVEFORM_WIDTH` writer - Pulse waveform width, low 8 bits
pub type PulseWaveformWidthW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl W {
    /// Bits 0:7 - Pulse waveform width, low 8 bits
    #[inline(always)]
    pub fn pulse_waveform_width(&mut self) -> PulseWaveformWidthW<'_, PwloSpec> {
        PulseWaveformWidthW::new(self, 0)
    }
}
/// Pulse width bits 7:0
///
/// You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwlo::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct PwloSpec;
impl crate::RegisterSpec for PwloSpec {
    type Ux = u8;
}
/// `write(|w| ..)` method takes [`pwlo::W`](W) writer structure
impl crate::Writable for PwloSpec {
    type Safety = crate::Unsafe;
}
/// `reset()` method sets PWLO to value 0
impl crate::Resettable for PwloSpec {}
