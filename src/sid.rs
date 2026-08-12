#[repr(C)]
#[derive(Debug)]
/// Register block
pub struct RegisterBlock {
    voice: [Voice; 3],
    cutlo: Cutlo,
    cuthi: Cuthi,
    reson: Reson,
    sigvol: Sigvol,
    potx: Potx,
    poty: Poty,
    random: Random,
    env3: Env3,
}
impl RegisterBlock {
    /// 0x00..0x15 - Voice %s
    #[inline(always)]
    pub const fn voice(&self, n: usize) -> &Voice { &self.voice[n] }

    /// Iterator for array of:
    /// 0x00..0x15 - Voice %s
    #[inline(always)]
    pub fn voice_iter(&self) -> impl Iterator<Item = &Voice> { self.voice.iter() }

    /// 0x15 - Filter cutoff bits 2:0 (bits 3-7 unused)
    #[inline(always)]
    pub const fn cutlo(&self) -> &Cutlo { &self.cutlo }

    /// 0x16 - Filter cutoff bits 10:3
    #[inline(always)]
    pub const fn cuthi(&self) -> &Cuthi { &self.cuthi }

    /// 0x17 - Filter resonance and routing
    #[inline(always)]
    pub const fn reson(&self) -> &Reson { &self.reson }

    /// 0x18 - Filter mode and master volume
    #[inline(always)]
    pub const fn sigvol(&self) -> &Sigvol { &self.sigvol }

    /// 0x19 - Paddle X digitized position (updated every 512 cycles)
    #[inline(always)]
    pub const fn potx(&self) -> &Potx { &self.potx }

    /// 0x1a - Paddle Y digitized position
    #[inline(always)]
    pub const fn poty(&self) -> &Poty { &self.poty }

    /// 0x1b - Voice 3 oscillator output (usable as entropy with noise waveform)
    #[inline(always)]
    pub const fn random(&self) -> &Random { &self.random }

    /// 0x1c - Voice 3 envelope output
    #[inline(always)]
    pub const fn env3(&self) -> &Env3 { &self.env3 }
}
/// Voice %s
pub use self::voice::Voice;
/// Cluster
/// Voice %s
pub mod voice;
/// CUTLO (w) register accessor: Filter cutoff bits 2:0 (bits 3-7 unused)
///
/// You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cutlo::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
/// For information about available fields see [`mod@cutlo`] module
#[doc(alias = "CUTLO")]
pub type Cutlo = crate::Reg<cutlo::CutloSpec>;
/// Filter cutoff bits 2:0 (bits 3-7 unused)
pub mod cutlo;
/// CUTHI (w) register accessor: Filter cutoff bits 10:3
///
/// You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cuthi::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
/// For information about available fields see [`mod@cuthi`] module
#[doc(alias = "CUTHI")]
pub type Cuthi = crate::Reg<cuthi::CuthiSpec>;
/// Filter cutoff bits 10:3
pub mod cuthi;
/// RESON (w) register accessor: Filter resonance and routing
///
/// You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reson::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
/// For information about available fields see [`mod@reson`] module
#[doc(alias = "RESON")]
pub type Reson = crate::Reg<reson::ResonSpec>;
/// Filter resonance and routing
pub mod reson;
/// SIGVOL (w) register accessor: Filter mode and master volume
///
/// You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sigvol::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
/// For information about available fields see [`mod@sigvol`] module
#[doc(alias = "SIGVOL")]
pub type Sigvol = crate::Reg<sigvol::SigvolSpec>;
/// Filter mode and master volume
pub mod sigvol;
/// POTX (r) register accessor: Paddle X digitized position (updated every 512
/// cycles)
///
/// You can [`read`](crate::Reg::read) this register and get [`potx::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
/// For information about available fields see [`mod@potx`] module
#[doc(alias = "POTX")]
pub type Potx = crate::Reg<potx::PotxSpec>;
/// Paddle X digitized position (updated every 512 cycles)
pub mod potx;
/// POTY (r) register accessor: Paddle Y digitized position
///
/// You can [`read`](crate::Reg::read) this register and get [`poty::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
/// For information about available fields see [`mod@poty`] module
#[doc(alias = "POTY")]
pub type Poty = crate::Reg<poty::PotySpec>;
/// Paddle Y digitized position
pub mod poty;
/// RANDOM (r) register accessor: Voice 3 oscillator output (usable as entropy
/// with noise waveform)
///
/// You can [`read`](crate::Reg::read) this register and get [`random::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
/// For information about available fields see [`mod@random`] module
#[doc(alias = "RANDOM")]
pub type Random = crate::Reg<random::RandomSpec>;
/// Voice 3 oscillator output (usable as entropy with noise waveform)
pub mod random;
/// ENV3 (r) register accessor: Voice 3 envelope output
///
/// You can [`read`](crate::Reg::read) this register and get [`env3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
/// For information about available fields see [`mod@env3`] module
#[doc(alias = "ENV3")]
pub type Env3 = crate::Reg<env3::Env3Spec>;
/// Voice 3 envelope output
pub mod env3;
