#[repr(C)]
#[derive(Debug)]
/// Register block
pub struct RegisterBlock {
    cram: [Cram; 1024],
}
impl RegisterBlock {
    /// 0x00..0x400 - Color cell (bits 3:0)
    #[inline(always)]
    pub const fn cram(&self, n: usize) -> &Cram { &self.cram[n] }

    /// Iterator for array of:
    /// 0x00..0x400 - Color cell (bits 3:0)
    #[inline(always)]
    pub fn cram_iter(&self) -> impl Iterator<Item = &Cram> { self.cram.iter() }
}
/// CRAM (rw) register accessor: Color cell (bits 3:0)
///
/// You can [`read`](crate::Reg::read) this register and get [`cram::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cram::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
/// For information about available fields see [`mod@cram`] module
#[doc(alias = "CRAM")]
pub type Cram = crate::Reg<cram::CramSpec>;
/// Color cell (bits 3:0)
pub mod cram;
