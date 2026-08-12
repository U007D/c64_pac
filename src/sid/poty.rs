/// Register `POTY` reader
pub type R = crate::R<PotySpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
/// Paddle Y digitized position
///
/// You can [`read`](crate::Reg::read) this register and get [`poty::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct PotySpec;
impl crate::RegisterSpec for PotySpec {
    type Ux = u8;
}
/// `read()` method returns [`poty::R`](R) reader structure
impl crate::Readable for PotySpec {}
/// `reset()` method sets POTY to value 0
impl crate::Resettable for PotySpec {}
