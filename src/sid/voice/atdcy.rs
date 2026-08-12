/// Register `ATDCY` writer
pub type W = crate::W<AtdcySpec>;
/// Field `DECAY` writer - Decay rate
pub type DecayW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
/// Field `ATTACK` writer - Attack rate
pub type AttackW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl W {
    /// Bits 0:3 - Decay rate
    #[inline(always)]
    pub fn decay(&mut self) -> DecayW<'_, AtdcySpec> { DecayW::new(self, 0) }

    /// Bits 4:7 - Attack rate
    #[inline(always)]
    pub fn attack(&mut self) -> AttackW<'_, AtdcySpec> { AttackW::new(self, 4) }
}
/// Envelope attack/decay
///
/// You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`atdcy::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct AtdcySpec;
impl crate::RegisterSpec for AtdcySpec {
    type Ux = u8;
}
/// `write(|w| ..)` method takes [`atdcy::W`](W) writer structure
impl crate::Writable for AtdcySpec {
    type Safety = crate::Unsafe;
}
/// `reset()` method sets ATDCY to value 0
impl crate::Resettable for AtdcySpec {}
