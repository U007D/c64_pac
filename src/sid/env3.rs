/// Register `ENV3` reader
pub type R = crate::R<Env3Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
/// Voice 3 envelope output
///
/// You can [`read`](crate::Reg::read) this register and get [`env3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct Env3Spec;
impl crate::RegisterSpec for Env3Spec {
    type Ux = u8;
}
/// `read()` method returns [`env3::R`](R) reader structure
impl crate::Readable for Env3Spec {}
/// `reset()` method sets ENV3 to value 0
impl crate::Resettable for Env3Spec {}
