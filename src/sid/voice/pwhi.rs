/// Register `PWHI` writer
pub type W = crate::W<PwhiSpec>;
/// Field `PULSE_WAVEFORM_WIDTH` writer - Pulse width bits 11:8
pub type PulseWaveformWidthW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl core::fmt::Debug for crate::generic::Reg<PwhiSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    /// Bits 0:3 - Pulse width bits 11:8
    #[inline(always)]
    pub fn pulse_waveform_width(&mut self) -> PulseWaveformWidthW<'_, PwhiSpec> {
        PulseWaveformWidthW::new(self, 0)
    }
}
/// Pulse width bits 11:8 (bits 4-7 unused)
///
/// You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwhi::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct PwhiSpec;
impl crate::RegisterSpec for PwhiSpec {
    type Ux = u8;
}
/// `write(|w| ..)` method takes [`pwhi::W`](W) writer structure
impl crate::Writable for PwhiSpec {
    type Safety = crate::Unsafe;
}
/// `reset()` method sets PWHI to value 0
impl crate::Resettable for PwhiSpec {}
