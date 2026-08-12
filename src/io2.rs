#[repr(C)]
/// Register block
pub struct RegisterBlock {
    byte: [Byte; 256],
}
impl RegisterBlock {
    /// 0x00..0x100 - Cartridge-defined
    #[inline(always)]
    pub const fn byte(&self, n: usize) -> &Byte { &self.byte[n] }

    /// Iterator for array of:
    /// 0x00..0x100 - Cartridge-defined
    #[inline(always)]
    pub fn byte_iter(&self) -> impl Iterator<Item = &Byte> { self.byte.iter() }
}
/// BYTE (rw) register accessor: Cartridge-defined
///
/// You can [`read`](crate::Reg::read) this register and get [`byte::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`byte::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
/// For information about available fields see [`mod@byte`] module
#[doc(alias = "BYTE")]
pub type Byte = crate::Reg<byte::ByteSpec>;
/// Cartridge-defined
pub mod byte;
