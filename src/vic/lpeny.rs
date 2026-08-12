/// Register `LPENY` reader
pub type R = crate::R<LpenySpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
/// Light pen Y
///
/// You can [`read`](crate::Reg::read) this register and get [`lpeny::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct LpenySpec;
impl crate::RegisterSpec for LpenySpec {
    type Ux = u8;
}
/// `read()` method returns [`lpeny::R`](R) reader structure
impl crate::Readable for LpenySpec {}
/// `reset()` method sets LPENY to value 0
impl crate::Resettable for LpenySpec {}
