/// Register `VMCSB` reader
pub type R = crate::R<VmcsbSpec>;
/// Register `VMCSB` writer
pub type W = crate::W<VmcsbSpec>;
/// Field `CHAR_BASE` reader - Character/bitmap base address bits 13:11
pub type CharBaseR = crate::FieldReader;
/// Field `CHAR_BASE` writer - Character/bitmap base address bits 13:11
pub type CharBaseW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
/// Field `VIDEO_MATRIX` reader - Video matrix (screen) base address bits 13:10
pub type VideoMatrixR = crate::FieldReader;
/// Field `VIDEO_MATRIX` writer - Video matrix (screen) base address bits 13:10
pub type VideoMatrixW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    /// Bits 1:3 - Character/bitmap base address bits 13:11
    #[inline(always)]
    pub fn char_base(&self) -> CharBaseR { CharBaseR::new((self.bits >> 1) & 7) }

    /// Bits 4:7 - Video matrix (screen) base address bits 13:10
    #[inline(always)]
    pub fn video_matrix(&self) -> VideoMatrixR { VideoMatrixR::new((self.bits >> 4) & 0x0f) }
}
impl W {
    /// Bits 1:3 - Character/bitmap base address bits 13:11
    #[inline(always)]
    pub fn char_base(&mut self) -> CharBaseW<'_, VmcsbSpec> { CharBaseW::new(self, 1) }

    /// Bits 4:7 - Video matrix (screen) base address bits 13:10
    #[inline(always)]
    pub fn video_matrix(&mut self) -> VideoMatrixW<'_, VmcsbSpec> { VideoMatrixW::new(self, 4) }
}
/// Memory pointers within the current VIC bank. Bit 0 unimplemented.
///
/// You can [`read`](crate::Reg::read) this register and get [`vmcsb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vmcsb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct VmcsbSpec;
impl crate::RegisterSpec for VmcsbSpec {
    type Ux = u8;
}
/// `read()` method returns [`vmcsb::R`](R) reader structure
impl crate::Readable for VmcsbSpec {}
/// `write(|w| ..)` method takes [`vmcsb::W`](W) writer structure
impl crate::Writable for VmcsbSpec {
    type Safety = crate::Unsafe;
}
/// `reset()` method sets VMCSB to value 0
impl crate::Resettable for VmcsbSpec {}
