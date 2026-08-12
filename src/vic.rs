#[repr(C)]
/// Register block
pub struct RegisterBlock {
    spx: (),
    _reserved1: [u8; 0x01],
    spy: (),
    _reserved2: [u8; 0x0f],
    msigx: Msigx,
    scroly: Scroly,
    _reserved_4_raster: [u8; 0x01],
    lpenx: Lpenx,
    lpeny: Lpeny,
    spena: Spena,
    scrolx: Scrolx,
    yxpand: Yxpand,
    vmcsb: Vmcsb,
    _reserved_11_vicirq: [u8; 0x01],
    irqmask: Irqmask,
    spbgpr: Spbgpr,
    spmc: Spmc,
    xxpand: Xxpand,
    spspcl: Spspcl,
    spbgcl: Spbgcl,
    extcol: Extcol,
    bgcol: [Bgcol; 4],
    spmc0: Spmc0,
    spmc1: Spmc1,
    spcol: [Spcol; 8],
}
impl RegisterBlock {
    /// 0x00..0x08 - Sprite X position bits 7:0 (bit 8 in MSIGX)
    #[inline(always)]
    pub const fn spx(&self, n: usize) -> &Spx {
        #[allow(clippy::no_effect)]
        [(); 8][n];
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(2 * n).cast() }
    }

    /// Iterator for array of:
    /// 0x00..0x08 - Sprite X position bits 7:0 (bit 8 in MSIGX)
    #[inline(always)]
    pub fn spx_iter(&self) -> impl Iterator<Item = &Spx> {
        (0..8).map(move |n| unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(2 * n).cast() })
    }

    /// 0x00 - Sprite X position bits 7:0 (bit 8 in MSIGX)
    #[inline(always)]
    pub const fn sp0x(&self) -> &Spx { self.spx(0) }

    /// 0x02 - Sprite X position bits 7:0 (bit 8 in MSIGX)
    #[inline(always)]
    pub const fn sp1x(&self) -> &Spx { self.spx(1) }

    /// 0x04 - Sprite X position bits 7:0 (bit 8 in MSIGX)
    #[inline(always)]
    pub const fn sp2x(&self) -> &Spx { self.spx(2) }

    /// 0x06 - Sprite X position bits 7:0 (bit 8 in MSIGX)
    #[inline(always)]
    pub const fn sp3x(&self) -> &Spx { self.spx(3) }

    /// 0x08 - Sprite X position bits 7:0 (bit 8 in MSIGX)
    #[inline(always)]
    pub const fn sp4x(&self) -> &Spx { self.spx(4) }

    /// 0x0a - Sprite X position bits 7:0 (bit 8 in MSIGX)
    #[inline(always)]
    pub const fn sp5x(&self) -> &Spx { self.spx(5) }

    /// 0x0c - Sprite X position bits 7:0 (bit 8 in MSIGX)
    #[inline(always)]
    pub const fn sp6x(&self) -> &Spx { self.spx(6) }

    /// 0x0e - Sprite X position bits 7:0 (bit 8 in MSIGX)
    #[inline(always)]
    pub const fn sp7x(&self) -> &Spx { self.spx(7) }

    /// 0x01..0x09 - Sprite Y position
    #[inline(always)]
    pub const fn spy(&self, n: usize) -> &Spy {
        #[allow(clippy::no_effect)]
        [(); 8][n];
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1).add(2 * n).cast() }
    }

    /// Iterator for array of:
    /// 0x01..0x09 - Sprite Y position
    #[inline(always)]
    pub fn spy_iter(&self) -> impl Iterator<Item = &Spy> {
        (0..8).map(move |n| unsafe {
                  &*core::ptr::from_ref(self).cast::<u8>().add(1).add(2 * n).cast()
              })
    }

    /// 0x01 - Sprite Y position
    #[inline(always)]
    pub const fn sp0y(&self) -> &Spy { self.spy(0) }

    /// 0x03 - Sprite Y position
    #[inline(always)]
    pub const fn sp1y(&self) -> &Spy { self.spy(1) }

    /// 0x05 - Sprite Y position
    #[inline(always)]
    pub const fn sp2y(&self) -> &Spy { self.spy(2) }

    /// 0x07 - Sprite Y position
    #[inline(always)]
    pub const fn sp3y(&self) -> &Spy { self.spy(3) }

    /// 0x09 - Sprite Y position
    #[inline(always)]
    pub const fn sp4y(&self) -> &Spy { self.spy(4) }

    /// 0x0b - Sprite Y position
    #[inline(always)]
    pub const fn sp5y(&self) -> &Spy { self.spy(5) }

    /// 0x0d - Sprite Y position
    #[inline(always)]
    pub const fn sp6y(&self) -> &Spy { self.spy(6) }

    /// 0x0f - Sprite Y position
    #[inline(always)]
    pub const fn sp7y(&self) -> &Spy { self.spy(7) }

    /// 0x10 - Sprite X position bit 8; bit n belongs to sprite n
    #[inline(always)]
    pub const fn msigx(&self) -> &Msigx { &self.msigx }

    /// 0x11 - Control register 1. RST8 reads current raster bit 8, writes
    /// raster-compare bit 8.
    #[inline(always)]
    pub const fn scroly(&self) -> &Scroly { &self.scroly }

    /// 0x12 - Raster line at which a raster IRQ fires (compare), bits 7:0 (bit
    /// 8 = SCROLY.RST8). Write-only alternate view of 0xD012; the current-line
    /// read view is RASTER_R.
    #[inline(always)]
    pub const fn raster_w(&self) -> &RasterW {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(18).cast() }
    }

    /// 0x12 - Current raster line, bits 7:0 (bit 8 = SCROLY.RST8). Read-only
    /// view of 0xD012. Read and write are different registers at this address;
    /// the write view is RASTER_W, so neither exposes `.modify()`.
    #[inline(always)]
    pub const fn raster_r(&self) -> &RasterR {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(18).cast() }
    }

    /// 0x13 - Light pen X (2-pixel resolution)
    #[inline(always)]
    pub const fn lpenx(&self) -> &Lpenx { &self.lpenx }

    /// 0x14 - Light pen Y
    #[inline(always)]
    pub const fn lpeny(&self) -> &Lpeny { &self.lpeny }

    /// 0x15 - Sprite enable, bit n = sprite n
    #[inline(always)]
    pub const fn spena(&self) -> &Spena { &self.spena }

    /// 0x16 - Control register 2. Bits 6-7 unimplemented, read as 1.
    #[inline(always)]
    pub const fn scrolx(&self) -> &Scrolx { &self.scrolx }

    /// 0x17 - Sprite Y expansion, bit n = sprite n
    #[inline(always)]
    pub const fn yxpand(&self) -> &Yxpand { &self.yxpand }

    /// 0x18 - Memory pointers within the current VIC bank. Bit 0 unimplemented.
    #[inline(always)]
    pub const fn vmcsb(&self) -> &Vmcsb { &self.vmcsb }

    /// 0x19 - Acknowledge latched VIC interrupts (write-only alternate view of
    /// 0xD019, write-1-to-clear). Writing Ack to a bit clears that source; Keep
    /// (0) leaves its flag pending. There is no read here - read status via
    /// VICIRQ_R. SIDE-EFFECT: an enabled source left un-acked re-asserts the
    /// IRQ immediately.
    #[inline(always)]
    pub const fn vicirq_w(&self) -> &VicirqW {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(25).cast() }
    }

    /// 0x19 - Interrupt latch (read-only view of 0xD019): which VIC sources
    /// have fired. Bit 7 reads Latched while any enabled source is latched;
    /// bits 4-6 read as 1. Acknowledge via VICIRQ_W.
    #[inline(always)]
    pub const fn vicirq_r(&self) -> &VicirqR {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(25).cast() }
    }

    /// 0x1a - Interrupt enable for the four VICIRQ_R sources (same bit
    /// positions)
    #[inline(always)]
    pub const fn irqmask(&self) -> &Irqmask { &self.irqmask }

    /// 0x1b - Sprite-to-background priority, bit n = sprite n (1 = background
    /// in front)
    #[inline(always)]
    pub const fn spbgpr(&self) -> &Spbgpr { &self.spbgpr }

    /// 0x1c - Sprite multicolor enable, bit n = sprite n
    #[inline(always)]
    pub const fn spmc(&self) -> &Spmc { &self.spmc }

    /// 0x1d - Sprite X expansion, bit n = sprite n
    #[inline(always)]
    pub const fn xxpand(&self) -> &Xxpand { &self.xxpand }

    /// 0x1e - Sprite-sprite collision latch. SIDE-EFFECT: reading clears the
    /// latch - read once and keep the value.
    #[inline(always)]
    pub const fn spspcl(&self) -> &Spspcl { &self.spspcl }

    /// 0x1f - Sprite-background collision latch. SIDE-EFFECT: reading clears
    /// the latch - read once and keep the value.
    #[inline(always)]
    pub const fn spbgcl(&self) -> &Spbgcl { &self.spbgcl }

    /// 0x20 - Border color (bits 3:0; upper bits read as 1)
    #[inline(always)]
    pub const fn extcol(&self) -> &Extcol { &self.extcol }

    /// 0x21 - Background color (bits 3:0)
    #[inline(always)]
    pub const fn bgcol(&self, n: usize) -> &Bgcol { &self.bgcol[n] }

    /// Iterator for array of:
    /// 0x21 - Background color (bits 3:0)
    #[inline(always)]
    pub fn bgcol_iter(&self) -> impl Iterator<Item = &Bgcol> { self.bgcol.iter() }

    /// 0x25 - Sprite multicolor 0 (bits 3:0)
    #[inline(always)]
    pub const fn spmc0(&self) -> &Spmc0 { &self.spmc0 }

    /// 0x26 - Sprite multicolor 1 (bits 3:0)
    #[inline(always)]
    pub const fn spmc1(&self) -> &Spmc1 { &self.spmc1 }

    /// 0x27..0x2f - Sprite color (bits 3:0)
    #[inline(always)]
    pub const fn spcol(&self, n: usize) -> &Spcol { &self.spcol[n] }

    /// Iterator for array of:
    /// 0x27..0x2f - Sprite color (bits 3:0)
    #[inline(always)]
    pub fn spcol_iter(&self) -> impl Iterator<Item = &Spcol> { self.spcol.iter() }

    /// 0x27 - Sprite color (bits 3:0)
    #[inline(always)]
    pub const fn sp0col(&self) -> &Spcol { self.spcol(0) }

    /// 0x28 - Sprite color (bits 3:0)
    #[inline(always)]
    pub const fn sp1col(&self) -> &Spcol { self.spcol(1) }

    /// 0x29 - Sprite color (bits 3:0)
    #[inline(always)]
    pub const fn sp2col(&self) -> &Spcol { self.spcol(2) }

    /// 0x2a - Sprite color (bits 3:0)
    #[inline(always)]
    pub const fn sp3col(&self) -> &Spcol { self.spcol(3) }

    /// 0x2b - Sprite color (bits 3:0)
    #[inline(always)]
    pub const fn sp4col(&self) -> &Spcol { self.spcol(4) }

    /// 0x2c - Sprite color (bits 3:0)
    #[inline(always)]
    pub const fn sp5col(&self) -> &Spcol { self.spcol(5) }

    /// 0x2d - Sprite color (bits 3:0)
    #[inline(always)]
    pub const fn sp6col(&self) -> &Spcol { self.spcol(6) }

    /// 0x2e - Sprite color (bits 3:0)
    #[inline(always)]
    pub const fn sp7col(&self) -> &Spcol { self.spcol(7) }
}
/// SPX (rw) register accessor: Sprite X position bits 7:0 (bit 8 in MSIGX)
///
/// You can [`read`](crate::Reg::read) this register and get [`spx::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spx::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
/// For information about available fields see [`mod@spx`] module
#[doc(alias = "SPX")]
pub type Spx = crate::Reg<spx::SpxSpec>;
/// Sprite X position bits 7:0 (bit 8 in MSIGX)
pub mod spx;
/// SPY (rw) register accessor: Sprite Y position
///
/// You can [`read`](crate::Reg::read) this register and get [`spy::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spy::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
/// For information about available fields see [`mod@spy`] module
#[doc(alias = "SPY")]
pub type Spy = crate::Reg<spy::SpySpec>;
/// Sprite Y position
pub mod spy;
/// MSIGX (rw) register accessor: Sprite X position bit 8; bit n belongs to
/// sprite n
///
/// You can [`read`](crate::Reg::read) this register and get [`msigx::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`msigx::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
/// For information about available fields see [`mod@msigx`] module
#[doc(alias = "MSIGX")]
pub type Msigx = crate::Reg<msigx::MsigxSpec>;
/// Sprite X position bit 8; bit n belongs to sprite n
pub mod msigx;
/// SCROLY (rw) register accessor: Control register 1. RST8 reads current raster
/// bit 8, writes raster-compare bit 8.
///
/// You can [`read`](crate::Reg::read) this register and get [`scroly::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scroly::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
/// For information about available fields see [`mod@scroly`] module
#[doc(alias = "SCROLY")]
pub type Scroly = crate::Reg<scroly::ScrolySpec>;
/// Control register 1. RST8 reads current raster bit 8, writes raster-compare
/// bit 8.
pub mod scroly;
/// RASTER_R (r) register accessor: Current raster line, bits 7:0 (bit 8 =
/// SCROLY.RST8). Read-only view of 0xD012. Read and write are different
/// registers at this address; the write view is RASTER_W, so neither exposes
/// `.modify()`.
///
/// You can [`read`](crate::Reg::read) this register and get [`raster_r::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
/// For information about available fields see [`mod@raster_r`] module
#[doc(alias = "RASTER_R")]
pub type RasterR = crate::Reg<raster_r::RasterRSpec>;
/// Current raster line, bits 7:0 (bit 8 = SCROLY.RST8). Read-only view of
/// 0xD012. Read and write are different registers at this address; the write
/// view is RASTER_W, so neither exposes `.modify()`.
pub mod raster_r;
/// RASTER_W (w) register accessor: Raster line at which a raster IRQ fires
/// (compare), bits 7:0 (bit 8 = SCROLY.RST8). Write-only alternate view of
/// 0xD012; the current-line read view is RASTER_R.
///
/// You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`raster_w::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
/// For information about available fields see [`mod@raster_w`] module
#[doc(alias = "RASTER_W")]
pub type RasterW = crate::Reg<raster_w::RasterWSpec>;
/// Raster line at which a raster IRQ fires (compare), bits 7:0 (bit 8 =
/// SCROLY.RST8). Write-only alternate view of 0xD012; the current-line read
/// view is RASTER_R.
pub mod raster_w;
/// LPENX (r) register accessor: Light pen X (2-pixel resolution)
///
/// You can [`read`](crate::Reg::read) this register and get [`lpenx::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
/// For information about available fields see [`mod@lpenx`] module
#[doc(alias = "LPENX")]
pub type Lpenx = crate::Reg<lpenx::LpenxSpec>;
/// Light pen X (2-pixel resolution)
pub mod lpenx;
/// LPENY (r) register accessor: Light pen Y
///
/// You can [`read`](crate::Reg::read) this register and get [`lpeny::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
/// For information about available fields see [`mod@lpeny`] module
#[doc(alias = "LPENY")]
pub type Lpeny = crate::Reg<lpeny::LpenySpec>;
/// Light pen Y
pub mod lpeny;
/// SPENA (rw) register accessor: Sprite enable, bit n = sprite n
///
/// You can [`read`](crate::Reg::read) this register and get [`spena::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spena::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
/// For information about available fields see [`mod@spena`] module
#[doc(alias = "SPENA")]
pub type Spena = crate::Reg<spena::SpenaSpec>;
/// Sprite enable, bit n = sprite n
pub mod spena;
/// SCROLX (rw) register accessor: Control register 2. Bits 6-7 unimplemented,
/// read as 1.
///
/// You can [`read`](crate::Reg::read) this register and get [`scrolx::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scrolx::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
/// For information about available fields see [`mod@scrolx`] module
#[doc(alias = "SCROLX")]
pub type Scrolx = crate::Reg<scrolx::ScrolxSpec>;
/// Control register 2. Bits 6-7 unimplemented, read as 1.
pub mod scrolx;
/// YXPAND (rw) register accessor: Sprite Y expansion, bit n = sprite n
///
/// You can [`read`](crate::Reg::read) this register and get [`yxpand::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`yxpand::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
/// For information about available fields see [`mod@yxpand`] module
#[doc(alias = "YXPAND")]
pub type Yxpand = crate::Reg<yxpand::YxpandSpec>;
/// Sprite Y expansion, bit n = sprite n
pub mod yxpand;
/// VMCSB (rw) register accessor: Memory pointers within the current VIC bank.
/// Bit 0 unimplemented.
///
/// You can [`read`](crate::Reg::read) this register and get [`vmcsb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vmcsb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
/// For information about available fields see [`mod@vmcsb`] module
#[doc(alias = "VMCSB")]
pub type Vmcsb = crate::Reg<vmcsb::VmcsbSpec>;
/// Memory pointers within the current VIC bank. Bit 0 unimplemented.
pub mod vmcsb;
/// VICIRQ_R (r) register accessor: Interrupt latch (read-only view of 0xD019):
/// which VIC sources have fired. Bit 7 reads Latched while any enabled source
/// is latched; bits 4-6 read as 1. Acknowledge via VICIRQ_W.
///
/// You can [`read`](crate::Reg::read) this register and get [`vicirq_r::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
/// For information about available fields see [`mod@vicirq_r`] module
#[doc(alias = "VICIRQ_R")]
pub type VicirqR = crate::Reg<vicirq_r::VicirqRSpec>;
/// Interrupt latch (read-only view of 0xD019): which VIC sources have fired.
/// Bit 7 reads Latched while any enabled source is latched; bits 4-6 read as 1.
/// Acknowledge via VICIRQ_W.
pub mod vicirq_r;
/// VICIRQ_W (w) register accessor: Acknowledge latched VIC interrupts
/// (write-only alternate view of 0xD019, write-1-to-clear). Writing Ack to a
/// bit clears that source; Keep (0) leaves its flag pending. There is no read
/// here - read status via VICIRQ_R. SIDE-EFFECT: an enabled source left
/// un-acked re-asserts the IRQ immediately.
///
/// You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vicirq_w::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
/// For information about available fields see [`mod@vicirq_w`] module
#[doc(alias = "VICIRQ_W")]
pub type VicirqW = crate::Reg<vicirq_w::VicirqWSpec>;
/// Acknowledge latched VIC interrupts (write-only alternate view of 0xD019,
/// write-1-to-clear). Writing Ack to a bit clears that source; Keep (0) leaves
/// its flag pending. There is no read here - read status via VICIRQ_R.
/// SIDE-EFFECT: an enabled source left un-acked re-asserts the IRQ immediately.
pub mod vicirq_w;
/// IRQMASK (rw) register accessor: Interrupt enable for the four VICIRQ_R
/// sources (same bit positions)
///
/// You can [`read`](crate::Reg::read) this register and get [`irqmask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irqmask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
/// For information about available fields see [`mod@irqmask`] module
#[doc(alias = "IRQMASK")]
pub type Irqmask = crate::Reg<irqmask::IrqmaskSpec>;
/// Interrupt enable for the four VICIRQ_R sources (same bit positions)
pub mod irqmask;
/// SPBGPR (rw) register accessor: Sprite-to-background priority, bit n = sprite
/// n (1 = background in front)
///
/// You can [`read`](crate::Reg::read) this register and get [`spbgpr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spbgpr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
/// For information about available fields see [`mod@spbgpr`] module
#[doc(alias = "SPBGPR")]
pub type Spbgpr = crate::Reg<spbgpr::SpbgprSpec>;
/// Sprite-to-background priority, bit n = sprite n (1 = background in front)
pub mod spbgpr;
/// SPMC (rw) register accessor: Sprite multicolor enable, bit n = sprite n
///
/// You can [`read`](crate::Reg::read) this register and get [`spmc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spmc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
/// For information about available fields see [`mod@spmc`] module
#[doc(alias = "SPMC")]
pub type Spmc = crate::Reg<spmc::SpmcSpec>;
/// Sprite multicolor enable, bit n = sprite n
pub mod spmc;
/// XXPAND (rw) register accessor: Sprite X expansion, bit n = sprite n
///
/// You can [`read`](crate::Reg::read) this register and get [`xxpand::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xxpand::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
/// For information about available fields see [`mod@xxpand`] module
#[doc(alias = "XXPAND")]
pub type Xxpand = crate::Reg<xxpand::XxpandSpec>;
/// Sprite X expansion, bit n = sprite n
pub mod xxpand;
/// SPSPCL (r) register accessor: Sprite-sprite collision latch. SIDE-EFFECT:
/// reading clears the latch - read once and keep the value.
///
/// You can [`read`](crate::Reg::read) this register and get [`spspcl::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
/// <div class="warning">The register is <b>cleared</b> (set to zero) following
/// a read operation.</div>
///
/// For information about available fields see [`mod@spspcl`] module
#[doc(alias = "SPSPCL")]
pub type Spspcl = crate::Reg<spspcl::SpspclSpec>;
/// Sprite-sprite collision latch. SIDE-EFFECT: reading clears the latch - read
/// once and keep the value.
pub mod spspcl;
/// SPBGCL (r) register accessor: Sprite-background collision latch.
/// SIDE-EFFECT: reading clears the latch - read once and keep the value.
///
/// You can [`read`](crate::Reg::read) this register and get [`spbgcl::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
/// <div class="warning">The register is <b>cleared</b> (set to zero) following
/// a read operation.</div>
///
/// For information about available fields see [`mod@spbgcl`] module
#[doc(alias = "SPBGCL")]
pub type Spbgcl = crate::Reg<spbgcl::SpbgclSpec>;
/// Sprite-background collision latch. SIDE-EFFECT: reading clears the latch -
/// read once and keep the value.
pub mod spbgcl;
/// EXTCOL (rw) register accessor: Border color (bits 3:0; upper bits read as 1)
///
/// You can [`read`](crate::Reg::read) this register and get [`extcol::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`extcol::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
/// For information about available fields see [`mod@extcol`] module
#[doc(alias = "EXTCOL")]
pub type Extcol = crate::Reg<extcol::ExtcolSpec>;
/// Border color (bits 3:0; upper bits read as 1)
pub mod extcol;
pub use extcol::Color;
/// BGCOL (rw) register accessor: Background color (bits 3:0)
///
/// You can [`read`](crate::Reg::read) this register and get [`bgcol::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgcol::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
/// For information about available fields see [`mod@bgcol`] module
#[doc(alias = "BGCOL")]
pub type Bgcol = crate::Reg<bgcol::BgcolSpec>;
/// Background color (bits 3:0)
pub mod bgcol;
/// SPMC0 (rw) register accessor: Sprite multicolor 0 (bits 3:0)
///
/// You can [`read`](crate::Reg::read) this register and get [`spmc0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spmc0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
/// For information about available fields see [`mod@spmc0`] module
#[doc(alias = "SPMC0")]
pub type Spmc0 = crate::Reg<spmc0::Spmc0Spec>;
/// Sprite multicolor 0 (bits 3:0)
pub mod spmc0;
/// SPMC1 (rw) register accessor: Sprite multicolor 1 (bits 3:0)
///
/// You can [`read`](crate::Reg::read) this register and get [`spmc1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spmc1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
/// For information about available fields see [`mod@spmc1`] module
#[doc(alias = "SPMC1")]
pub type Spmc1 = crate::Reg<spmc1::Spmc1Spec>;
/// Sprite multicolor 1 (bits 3:0)
pub mod spmc1;
/// SPCOL (rw) register accessor: Sprite color (bits 3:0)
///
/// You can [`read`](crate::Reg::read) this register and get [`spcol::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spcol::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
/// For information about available fields see [`mod@spcol`] module
#[doc(alias = "SPCOL")]
pub type Spcol = crate::Reg<spcol::SpcolSpec>;
/// Sprite color (bits 3:0)
pub mod spcol;
