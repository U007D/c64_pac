/// Register `CI2PRA` reader
pub type R = crate::R<Ci2praSpec>;
/// Register `CI2PRA` writer
pub type W = crate::W<Ci2praSpec>;
/// VIC-II 16 KiB video bank: Bank0 = 0x0000-0x3FFF, Bank1 = 0x4000-0x7FFF,
/// Bank2 = 0x8000-0xBFFF, Bank3 = 0xC000-0xFFFF. The raw CIA2 bits are inverted
/// (Bank0 = 0b11 down to Bank3 = 0b00); the enum hides that so you pick a bank
/// by number.
///
/// Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Bank {
    /// 0: `0`
    Bank3 = 0,
    /// 1: `1`
    Bank2 = 1,
    /// 2: `10`
    Bank1 = 2,
    /// 3: `11`
    Bank0 = 3,
}
impl From<Bank> for u8 {
    #[inline(always)]
    fn from(variant: Bank) -> Self { variant as _ }
}
impl crate::FieldSpec for Bank {
    type Ux = u8;
}
impl crate::IsEnum for Bank {}
/// Field `VICBANK` reader - VIC-II 16 KiB video bank: Bank0 = 0x0000-0x3FFF,
/// Bank1 = 0x4000-0x7FFF, Bank2 = 0x8000-0xBFFF, Bank3 = 0xC000-0xFFFF. The raw
/// CIA2 bits are inverted (Bank0 = 0b11 down to Bank3 = 0b00); the enum hides
/// that so you pick a bank by number.
pub type VicbankR = crate::FieldReader<Bank>;
impl VicbankR {
    /// Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Bank {
        match self.bits {
            0 => Bank::Bank3,
            1 => Bank::Bank2,
            2 => Bank::Bank1,
            3 => Bank::Bank0,
            _ => unreachable!(),
        }
    }

    /// `0`
    #[inline(always)]
    pub fn is_bank3(&self) -> bool { *self == Bank::Bank3 }

    /// `1`
    #[inline(always)]
    pub fn is_bank2(&self) -> bool { *self == Bank::Bank2 }

    /// `10`
    #[inline(always)]
    pub fn is_bank1(&self) -> bool { *self == Bank::Bank1 }

    /// `11`
    #[inline(always)]
    pub fn is_bank0(&self) -> bool { *self == Bank::Bank0 }
}
/// Field `VICBANK` writer - VIC-II 16 KiB video bank: Bank0 = 0x0000-0x3FFF,
/// Bank1 = 0x4000-0x7FFF, Bank2 = 0x8000-0xBFFF, Bank3 = 0xC000-0xFFFF. The raw
/// CIA2 bits are inverted (Bank0 = 0b11 down to Bank3 = 0b00); the enum hides
/// that so you pick a bank by number.
pub type VicbankW<'a, REG> = crate::FieldWriter<'a, REG, 2, Bank, crate::Safe>;
impl<'a, REG> VicbankW<'a, REG>
    where REG: crate::Writable + crate::RegisterSpec,
          REG::Ux: From<u8>,
{
    /// `0`
    #[inline(always)]
    pub fn bank3(self) -> &'a mut crate::W<REG> { self.variant(Bank::Bank3) }

    /// `1`
    #[inline(always)]
    pub fn bank2(self) -> &'a mut crate::W<REG> { self.variant(Bank::Bank2) }

    /// `10`
    #[inline(always)]
    pub fn bank1(self) -> &'a mut crate::W<REG> { self.variant(Bank::Bank1) }

    /// `11`
    #[inline(always)]
    pub fn bank0(self) -> &'a mut crate::W<REG> { self.variant(Bank::Bank0) }
}
/// Field `TXD` reader - RS-232 TXD (user port pin M)
pub type TxdR = crate::BitReader;
/// Field `TXD` writer - RS-232 TXD (user port pin M)
pub type TxdW<'a, REG> = crate::BitWriter<'a, REG>;
/// Field `ATN_OUT` reader - IEC ATN out (inverted onto bus)
pub type AtnOutR = crate::BitReader;
/// Field `ATN_OUT` writer - IEC ATN out (inverted onto bus)
pub type AtnOutW<'a, REG> = crate::BitWriter<'a, REG>;
/// Field `CLK_OUT` reader - IEC CLK out (inverted onto bus)
pub type ClkOutR = crate::BitReader;
/// Field `CLK_OUT` writer - IEC CLK out (inverted onto bus)
pub type ClkOutW<'a, REG> = crate::BitWriter<'a, REG>;
/// Field `DATA_OUT` reader - IEC DATA out (inverted onto bus)
pub type DataOutR = crate::BitReader;
/// Field `DATA_OUT` writer - IEC DATA out (inverted onto bus)
pub type DataOutW<'a, REG> = crate::BitWriter<'a, REG>;
/// Field `CLK_IN` reader - IEC CLK in
pub type ClkInR = crate::BitReader;
/// Field `DATA_IN` reader - IEC DATA in
pub type DataInR = crate::BitReader;
impl R {
    /// Bits 0:1 - VIC-II 16 KiB video bank: Bank0 = 0x0000-0x3FFF, Bank1 =
    /// 0x4000-0x7FFF, Bank2 = 0x8000-0xBFFF, Bank3 = 0xC000-0xFFFF. The raw
    /// CIA2 bits are inverted (Bank0 = 0b11 down to Bank3 = 0b00); the enum
    /// hides that so you pick a bank by number.
    #[inline(always)]
    pub fn vicbank(&self) -> VicbankR { VicbankR::new(self.bits & 3) }

    /// Bit 2 - RS-232 TXD (user port pin M)
    #[inline(always)]
    pub fn txd(&self) -> TxdR { TxdR::new(((self.bits >> 2) & 1) != 0) }

    /// Bit 3 - IEC ATN out (inverted onto bus)
    #[inline(always)]
    pub fn atn_out(&self) -> AtnOutR { AtnOutR::new(((self.bits >> 3) & 1) != 0) }

    /// Bit 4 - IEC CLK out (inverted onto bus)
    #[inline(always)]
    pub fn clk_out(&self) -> ClkOutR { ClkOutR::new(((self.bits >> 4) & 1) != 0) }

    /// Bit 5 - IEC DATA out (inverted onto bus)
    #[inline(always)]
    pub fn data_out(&self) -> DataOutR { DataOutR::new(((self.bits >> 5) & 1) != 0) }

    /// Bit 6 - IEC CLK in
    #[inline(always)]
    pub fn clk_in(&self) -> ClkInR { ClkInR::new(((self.bits >> 6) & 1) != 0) }

    /// Bit 7 - IEC DATA in
    #[inline(always)]
    pub fn data_in(&self) -> DataInR { DataInR::new(((self.bits >> 7) & 1) != 0) }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CI2PRA")
         .field("vicbank", &self.vicbank())
         .field("txd", &self.txd())
         .field("atn_out", &self.atn_out())
         .field("clk_out", &self.clk_out())
         .field("data_out", &self.data_out())
         .field("clk_in", &self.clk_in())
         .field("data_in", &self.data_in())
         .finish()
    }
}
impl W {
    /// Bits 0:1 - VIC-II 16 KiB video bank: Bank0 = 0x0000-0x3FFF, Bank1 =
    /// 0x4000-0x7FFF, Bank2 = 0x8000-0xBFFF, Bank3 = 0xC000-0xFFFF. The raw
    /// CIA2 bits are inverted (Bank0 = 0b11 down to Bank3 = 0b00); the enum
    /// hides that so you pick a bank by number.
    #[inline(always)]
    pub fn vicbank(&mut self) -> VicbankW<'_, Ci2praSpec> { VicbankW::new(self, 0) }

    /// Bit 2 - RS-232 TXD (user port pin M)
    #[inline(always)]
    pub fn txd(&mut self) -> TxdW<'_, Ci2praSpec> { TxdW::new(self, 2) }

    /// Bit 3 - IEC ATN out (inverted onto bus)
    #[inline(always)]
    pub fn atn_out(&mut self) -> AtnOutW<'_, Ci2praSpec> { AtnOutW::new(self, 3) }

    /// Bit 4 - IEC CLK out (inverted onto bus)
    #[inline(always)]
    pub fn clk_out(&mut self) -> ClkOutW<'_, Ci2praSpec> { ClkOutW::new(self, 4) }

    /// Bit 5 - IEC DATA out (inverted onto bus)
    #[inline(always)]
    pub fn data_out(&mut self) -> DataOutW<'_, Ci2praSpec> { DataOutW::new(self, 5) }
}
/// Port A: VIC bank, IEC bus, RS-232 TXD. IEC outputs pass through inverting
/// open-collector drivers: writing 1 pulls the bus line low. Inputs read the
/// bus level directly.
///
/// You can [`read`](crate::Reg::read) this register and get [`ci2pra::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ci2pra::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct Ci2praSpec;
impl crate::RegisterSpec for Ci2praSpec {
    type Ux = u8;
}
/// `read()` method returns [`ci2pra::R`](R) reader structure
impl crate::Readable for Ci2praSpec {}
/// `write(|w| ..)` method takes [`ci2pra::W`](W) writer structure
impl crate::Writable for Ci2praSpec {
    type Safety = crate::Unsafe;
}
/// `reset()` method sets CI2PRA to value 0
impl crate::Resettable for Ci2praSpec {}
