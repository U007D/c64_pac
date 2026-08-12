/// Register `CI2PRB` reader
pub type R = crate::R<Ci2prbSpec>;
/// Register `CI2PRB` writer
pub type W = crate::W<Ci2prbSpec>;
/// Field `SIN_PIN_C` reader - RS-232 data input (SIN) / user port pin C
pub type SinPinCR = crate::BitReader;
/// Field `SIN_PIN_C` writer - RS-232 data input (SIN) / user port pin C
pub type SinPinCW<'a, REG> = crate::BitWriter<'a, REG>;
/// Field `RTS_PIN_D` reader - RS-232 request to send (RTS) / user port pin D
pub type RtsPinDR = crate::BitReader;
/// Field `RTS_PIN_D` writer - RS-232 request to send (RTS) / user port pin D
pub type RtsPinDW<'a, REG> = crate::BitWriter<'a, REG>;
/// Field `DTR_PIN_E` reader - RS-232 data terminal ready (DTR) / user port pin
/// E
pub type DtrPinER = crate::BitReader;
/// Field `DTR_PIN_E` writer - RS-232 data terminal ready (DTR) / user port pin
/// E
pub type DtrPinEW<'a, REG> = crate::BitWriter<'a, REG>;
/// Field `RI_PIN_F` reader - RS-232 ring indicator (RI) / user port pin F
pub type RiPinFR = crate::BitReader;
/// Field `RI_PIN_F` writer - RS-232 ring indicator (RI) / user port pin F
pub type RiPinFW<'a, REG> = crate::BitWriter<'a, REG>;
/// Field `DCD_PIN_H` reader - RS-232 carrier detect (DCD) / user port pin H
pub type DcdPinHR = crate::BitReader;
/// Field `DCD_PIN_H` writer - RS-232 carrier detect (DCD) / user port pin H
pub type DcdPinHW<'a, REG> = crate::BitWriter<'a, REG>;
/// Field `PIN_J` reader - User port pin J
pub type PinJR = crate::BitReader;
/// Field `PIN_J` writer - User port pin J
pub type PinJW<'a, REG> = crate::BitWriter<'a, REG>;
/// Field `CTS_PIN_K` reader - RS-232 clear to send (CTS) / user port pin K;
/// also Timer A toggle/pulse output
pub type CtsPinKR = crate::BitReader;
/// Field `CTS_PIN_K` writer - RS-232 clear to send (CTS) / user port pin K;
/// also Timer A toggle/pulse output
pub type CtsPinKW<'a, REG> = crate::BitWriter<'a, REG>;
/// Field `DSR_PIN_L` reader - RS-232 data set ready (DSR) / user port pin L;
/// also Timer B toggle/pulse output
pub type DsrPinLR = crate::BitReader;
/// Field `DSR_PIN_L` writer - RS-232 data set ready (DSR) / user port pin L;
/// also Timer B toggle/pulse output
pub type DsrPinLW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    /// Bit 0 - RS-232 data input (SIN) / user port pin C
    #[inline(always)]
    pub fn sin_pin_c(&self) -> SinPinCR { SinPinCR::new((self.bits & 1) != 0) }

    /// Bit 1 - RS-232 request to send (RTS) / user port pin D
    #[inline(always)]
    pub fn rts_pin_d(&self) -> RtsPinDR { RtsPinDR::new(((self.bits >> 1) & 1) != 0) }

    /// Bit 2 - RS-232 data terminal ready (DTR) / user port pin E
    #[inline(always)]
    pub fn dtr_pin_e(&self) -> DtrPinER { DtrPinER::new(((self.bits >> 2) & 1) != 0) }

    /// Bit 3 - RS-232 ring indicator (RI) / user port pin F
    #[inline(always)]
    pub fn ri_pin_f(&self) -> RiPinFR { RiPinFR::new(((self.bits >> 3) & 1) != 0) }

    /// Bit 4 - RS-232 carrier detect (DCD) / user port pin H
    #[inline(always)]
    pub fn dcd_pin_h(&self) -> DcdPinHR { DcdPinHR::new(((self.bits >> 4) & 1) != 0) }

    /// Bit 5 - User port pin J
    #[inline(always)]
    pub fn pin_j(&self) -> PinJR { PinJR::new(((self.bits >> 5) & 1) != 0) }

    /// Bit 6 - RS-232 clear to send (CTS) / user port pin K; also Timer A
    /// toggle/pulse output
    #[inline(always)]
    pub fn cts_pin_k(&self) -> CtsPinKR { CtsPinKR::new(((self.bits >> 6) & 1) != 0) }

    /// Bit 7 - RS-232 data set ready (DSR) / user port pin L; also Timer B
    /// toggle/pulse output
    #[inline(always)]
    pub fn dsr_pin_l(&self) -> DsrPinLR { DsrPinLR::new(((self.bits >> 7) & 1) != 0) }
}
impl W {
    /// Bit 0 - RS-232 data input (SIN) / user port pin C
    #[inline(always)]
    pub fn sin_pin_c(&mut self) -> SinPinCW<'_, Ci2prbSpec> { SinPinCW::new(self, 0) }

    /// Bit 1 - RS-232 request to send (RTS) / user port pin D
    #[inline(always)]
    pub fn rts_pin_d(&mut self) -> RtsPinDW<'_, Ci2prbSpec> { RtsPinDW::new(self, 1) }

    /// Bit 2 - RS-232 data terminal ready (DTR) / user port pin E
    #[inline(always)]
    pub fn dtr_pin_e(&mut self) -> DtrPinEW<'_, Ci2prbSpec> { DtrPinEW::new(self, 2) }

    /// Bit 3 - RS-232 ring indicator (RI) / user port pin F
    #[inline(always)]
    pub fn ri_pin_f(&mut self) -> RiPinFW<'_, Ci2prbSpec> { RiPinFW::new(self, 3) }

    /// Bit 4 - RS-232 carrier detect (DCD) / user port pin H
    #[inline(always)]
    pub fn dcd_pin_h(&mut self) -> DcdPinHW<'_, Ci2prbSpec> { DcdPinHW::new(self, 4) }

    /// Bit 5 - User port pin J
    #[inline(always)]
    pub fn pin_j(&mut self) -> PinJW<'_, Ci2prbSpec> { PinJW::new(self, 5) }

    /// Bit 6 - RS-232 clear to send (CTS) / user port pin K; also Timer A
    /// toggle/pulse output
    #[inline(always)]
    pub fn cts_pin_k(&mut self) -> CtsPinKW<'_, Ci2prbSpec> { CtsPinKW::new(self, 6) }

    /// Bit 7 - RS-232 data set ready (DSR) / user port pin L; also Timer B
    /// toggle/pulse output
    #[inline(always)]
    pub fn dsr_pin_l(&mut self) -> DsrPinLW<'_, Ci2prbSpec> { DsrPinLW::new(self, 7) }
}
/// Port B: user port PB0-PB7 (pins C-L). KERNAL RS-232 assigns RXD to PB0
/// (paralleled with FLAG2) and handshake lines to PB1-PB7. General-purpose GPIO
/// when RS-232 is unused.
///
/// You can [`read`](crate::Reg::read) this register and get [`ci2prb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ci2prb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct Ci2prbSpec;
impl crate::RegisterSpec for Ci2prbSpec {
    type Ux = u8;
}
/// `read()` method returns [`ci2prb::R`](R) reader structure
impl crate::Readable for Ci2prbSpec {}
/// `write(|w| ..)` method takes [`ci2prb::W`](W) writer structure
impl crate::Writable for Ci2prbSpec {
    type Safety = crate::Unsafe;
}
/// `reset()` method sets CI2PRB to value 0
impl crate::Resettable for Ci2prbSpec {}
