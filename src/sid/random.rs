/// Register `RANDOM` reader
pub type R = crate::R<RandomSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
/// Voice 3 oscillator output (usable as entropy with noise waveform)
///
/// You can [`read`](crate::Reg::read) this register and get [`random::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct RandomSpec;
impl crate::RegisterSpec for RandomSpec {
    type Ux = u8;
}
/// `read()` method returns [`random::R`](R) reader structure
impl crate::Readable for RandomSpec {}
/// `reset()` method sets RANDOM to value 0
impl crate::Resettable for RandomSpec {}
