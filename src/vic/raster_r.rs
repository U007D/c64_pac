/// Register `RASTER_R` reader
pub type R = crate::R<RasterRSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
/// Current raster line, bits 7:0 (bit 8 = SCROLY.RST8). Read-only view of
/// 0xD012. Read and write are different registers at this address; the write
/// view is RASTER_W, so neither exposes `.modify()`.
///
/// You can [`read`](crate::Reg::read) this register and get [`raster_r::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct RasterRSpec;
impl crate::RegisterSpec for RasterRSpec {
    type Ux = u8;
}
/// `read()` method returns [`raster_r::R`](R) reader structure
impl crate::Readable for RasterRSpec {}
/// `reset()` method sets RASTER_R to value 0
impl crate::Resettable for RasterRSpec {}
