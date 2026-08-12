/// Register `LPENX` reader
pub type R = crate::R<LpenxSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
/// Light pen X (2-pixel resolution)
///
/// You can [`read`](crate::Reg::read) this register and get [`lpenx::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct LpenxSpec;
impl crate::RegisterSpec for LpenxSpec {
    type Ux = u8;
}
/// `read()` method returns [`lpenx::R`](R) reader structure
impl crate::Readable for LpenxSpec {}
/// `reset()` method sets LPENX to value 0
impl crate::Resettable for LpenxSpec {}
