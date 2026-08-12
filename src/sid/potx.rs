/// Register `POTX` reader
pub type R = crate::R<PotxSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
/// Paddle X digitized position (updated every 512 cycles)
///
/// You can [`read`](crate::Reg::read) this register and get [`potx::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct PotxSpec;
impl crate::RegisterSpec for PotxSpec {
    type Ux = u8;
}
/// `read()` method returns [`potx::R`](R) reader structure
impl crate::Readable for PotxSpec {}
/// `reset()` method sets POTX to value 0
impl crate::Resettable for PotxSpec {}
