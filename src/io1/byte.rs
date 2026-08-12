/// Register `BYTE%s` reader
pub type R = crate::R<ByteSpec>;
/// Register `BYTE%s` writer
pub type W = crate::W<ByteSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
/// Cartridge-defined
///
/// You can [`read`](crate::Reg::read) this register and get [`byte::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`byte::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct ByteSpec;
impl crate::RegisterSpec for ByteSpec {
    type Ux = u8;
}
/// `read()` method returns [`byte::R`](R) reader structure
impl crate::Readable for ByteSpec {}
/// `write(|w| ..)` method takes [`byte::W`](W) writer structure
impl crate::Writable for ByteSpec {
    type Safety = crate::Unsafe;
}
/// `reset()` method sets BYTE%s to value 0
impl crate::Resettable for ByteSpec {}
