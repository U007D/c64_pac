/// Register `RASTER_W` writer
pub type W = crate::W<RasterWSpec>;
/// Field `RASTER_LINE` writer - Raster line at which a raster IRQ fires, low 8
/// bits (bit 8 in SCROLY.RST8)
pub type RasterLineW<'a, REG> = crate::FieldWriter<'a, REG, 8, u8, crate::Safe>;
impl core::fmt::Debug for crate::generic::Reg<RasterWSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    /// Bits 0:7 - Raster line at which a raster IRQ fires, low 8 bits (bit 8 in
    /// SCROLY.RST8)
    #[inline(always)]
    pub fn raster_line(&mut self) -> RasterLineW<'_, RasterWSpec> { RasterLineW::new(self, 0) }
}
/// Raster line at which a raster IRQ fires (compare), bits 7:0 (bit 8 =
/// SCROLY.RST8). Write-only alternate view of 0xD012; the current-line read
/// view is RASTER_R.
///
/// You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`raster_w::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct RasterWSpec;
impl crate::RegisterSpec for RasterWSpec {
    type Ux = u8;
}
/// `write(|w| ..)` method takes [`raster_w::W`](W) writer structure
impl crate::Writable for RasterWSpec {
    type Safety = crate::Safe;
}
/// `reset()` method sets RASTER_W to value 0
impl crate::Resettable for RasterWSpec {}
