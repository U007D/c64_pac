/// Register `VMCSB` reader
pub type R = crate::R<VmcsbSpec>;
/// Register `VMCSB` writer
pub type W = crate::W<VmcsbSpec>;
/// Character/bitmap base, in 2 KiB steps from the base of the current VIC bank (register bits 3:1). [Mapping the C64](https://archive.org/details/Compute_s_Mapping_the_Commodore_64) quotes this as a whole-register value; halve the book value to get the field value, or use the named offsets below.
///
/// Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CharBase {
    /// 0: Character base at offset 0x0000 within the VIC bank
    Offset0000 = 0,
    /// 1: Character base at offset 0x0800 within the VIC bank
    Offset0800 = 1,
    /// 2: Character base at offset 0x1000 within the VIC bank - reads the
    /// character generator ROM, not RAM, in VIC banks 0 and 2
    Offset1000 = 2,
    /// 3: Character base at offset 0x1800 within the VIC bank - reads the
    /// character generator ROM, not RAM, in VIC banks 0 and 2
    Offset1800 = 3,
    /// 4: Character base at offset 0x2000 within the VIC bank
    Offset2000 = 4,
    /// 5: Character base at offset 0x2800 within the VIC bank
    Offset2800 = 5,
    /// 6: Character base at offset 0x3000 within the VIC bank
    Offset3000 = 6,
    /// 7: Character base at offset 0x3800 within the VIC bank
    Offset3800 = 7,
}
impl From<CharBase> for u8 {
    #[inline(always)]
    fn from(variant: CharBase) -> Self { variant as _ }
}
impl crate::FieldSpec for CharBase {
    type Ux = u8;
}
impl crate::IsEnum for CharBase {}
/// Field `CHAR_BASE` reader - Character/bitmap base, in 2 KiB steps from the base of the current VIC bank (register bits 3:1). [Mapping the C64](https://archive.org/details/Compute_s_Mapping_the_Commodore_64) quotes this as a whole-register value; halve the book value to get the field value, or use the named offsets below.
pub type CharBaseR = crate::FieldReader<CharBase>;
impl CharBaseR {
    /// Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CharBase {
        match self.bits {
            0 => CharBase::Offset0000,
            1 => CharBase::Offset0800,
            2 => CharBase::Offset1000,
            3 => CharBase::Offset1800,
            4 => CharBase::Offset2000,
            5 => CharBase::Offset2800,
            6 => CharBase::Offset3000,
            7 => CharBase::Offset3800,
            _ => unreachable!(),
        }
    }

    /// Character base at offset 0x0000 within the VIC bank
    #[inline(always)]
    pub fn is_offset_0000(&self) -> bool { *self == CharBase::Offset0000 }

    /// Character base at offset 0x0800 within the VIC bank
    #[inline(always)]
    pub fn is_offset_0800(&self) -> bool { *self == CharBase::Offset0800 }

    /// Character base at offset 0x1000 within the VIC bank - reads the
    /// character generator ROM, not RAM, in VIC banks 0 and 2
    #[inline(always)]
    pub fn is_offset_1000(&self) -> bool { *self == CharBase::Offset1000 }

    /// Character base at offset 0x1800 within the VIC bank - reads the
    /// character generator ROM, not RAM, in VIC banks 0 and 2
    #[inline(always)]
    pub fn is_offset_1800(&self) -> bool { *self == CharBase::Offset1800 }

    /// Character base at offset 0x2000 within the VIC bank
    #[inline(always)]
    pub fn is_offset_2000(&self) -> bool { *self == CharBase::Offset2000 }

    /// Character base at offset 0x2800 within the VIC bank
    #[inline(always)]
    pub fn is_offset_2800(&self) -> bool { *self == CharBase::Offset2800 }

    /// Character base at offset 0x3000 within the VIC bank
    #[inline(always)]
    pub fn is_offset_3000(&self) -> bool { *self == CharBase::Offset3000 }

    /// Character base at offset 0x3800 within the VIC bank
    #[inline(always)]
    pub fn is_offset_3800(&self) -> bool { *self == CharBase::Offset3800 }
}
/// Field `CHAR_BASE` writer - Character/bitmap base, in 2 KiB steps from the base of the current VIC bank (register bits 3:1). [Mapping the C64](https://archive.org/details/Compute_s_Mapping_the_Commodore_64) quotes this as a whole-register value; halve the book value to get the field value, or use the named offsets below.
pub type CharBaseW<'a, REG> = crate::FieldWriter<'a, REG, 3, CharBase, crate::Safe>;
impl<'a, REG> CharBaseW<'a, REG>
    where REG: crate::Writable + crate::RegisterSpec,
          REG::Ux: From<u8>,
{
    /// Character base at offset 0x0000 within the VIC bank
    #[inline(always)]
    pub fn offset_0000(self) -> &'a mut crate::W<REG> { self.variant(CharBase::Offset0000) }

    /// Character base at offset 0x0800 within the VIC bank
    #[inline(always)]
    pub fn offset_0800(self) -> &'a mut crate::W<REG> { self.variant(CharBase::Offset0800) }

    /// Character base at offset 0x1000 within the VIC bank - reads the
    /// character generator ROM, not RAM, in VIC banks 0 and 2
    #[inline(always)]
    pub fn offset_1000(self) -> &'a mut crate::W<REG> { self.variant(CharBase::Offset1000) }

    /// Character base at offset 0x1800 within the VIC bank - reads the
    /// character generator ROM, not RAM, in VIC banks 0 and 2
    #[inline(always)]
    pub fn offset_1800(self) -> &'a mut crate::W<REG> { self.variant(CharBase::Offset1800) }

    /// Character base at offset 0x2000 within the VIC bank
    #[inline(always)]
    pub fn offset_2000(self) -> &'a mut crate::W<REG> { self.variant(CharBase::Offset2000) }

    /// Character base at offset 0x2800 within the VIC bank
    #[inline(always)]
    pub fn offset_2800(self) -> &'a mut crate::W<REG> { self.variant(CharBase::Offset2800) }

    /// Character base at offset 0x3000 within the VIC bank
    #[inline(always)]
    pub fn offset_3000(self) -> &'a mut crate::W<REG> { self.variant(CharBase::Offset3000) }

    /// Character base at offset 0x3800 within the VIC bank
    #[inline(always)]
    pub fn offset_3800(self) -> &'a mut crate::W<REG> { self.variant(CharBase::Offset3800) }
}
/// Video matrix (screen) base, in 1 KiB steps from the base of the current VIC bank (register bits 7:4). [Mapping the C64](https://archive.org/details/Compute_s_Mapping_the_Commodore_64) quotes this as a whole-register value; divide the book value by 16 to get the field value, or use the named offsets below.
///
/// Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MatrixBase {
    /// 0: Screen matrix at offset 0x0000 within the VIC bank
    Offset0000 = 0,
    /// 1: Screen matrix at offset 0x0400 within the VIC bank
    Offset0400 = 1,
    /// 2: Screen matrix at offset 0x0800 within the VIC bank
    Offset0800 = 2,
    /// 3: Screen matrix at offset 0x0C00 within the VIC bank
    Offset0c00 = 3,
    /// 4: Screen matrix at offset 0x1000 within the VIC bank
    Offset1000 = 4,
    /// 5: Screen matrix at offset 0x1400 within the VIC bank
    Offset1400 = 5,
    /// 6: Screen matrix at offset 0x1800 within the VIC bank
    Offset1800 = 6,
    /// 7: Screen matrix at offset 0x1C00 within the VIC bank
    Offset1c00 = 7,
    /// 8: Screen matrix at offset 0x2000 within the VIC bank
    Offset2000 = 8,
    /// 9: Screen matrix at offset 0x2400 within the VIC bank
    Offset2400 = 9,
    /// 10: Screen matrix at offset 0x2800 within the VIC bank
    Offset2800 = 10,
    /// 11: Screen matrix at offset 0x2C00 within the VIC bank
    Offset2c00 = 11,
    /// 12: Screen matrix at offset 0x3000 within the VIC bank
    Offset3000 = 12,
    /// 13: Screen matrix at offset 0x3400 within the VIC bank
    Offset3400 = 13,
    /// 14: Screen matrix at offset 0x3800 within the VIC bank
    Offset3800 = 14,
    /// 15: Screen matrix at offset 0x3C00 within the VIC bank
    Offset3c00 = 15,
}
impl From<MatrixBase> for u8 {
    #[inline(always)]
    fn from(variant: MatrixBase) -> Self { variant as _ }
}
impl crate::FieldSpec for MatrixBase {
    type Ux = u8;
}
impl crate::IsEnum for MatrixBase {}
/// Field `VIDEO_MATRIX` reader - Video matrix (screen) base, in 1 KiB steps from the base of the current VIC bank (register bits 7:4). [Mapping the C64](https://archive.org/details/Compute_s_Mapping_the_Commodore_64) quotes this as a whole-register value; divide the book value by 16 to get the field value, or use the named offsets below.
pub type VideoMatrixR = crate::FieldReader<MatrixBase>;
impl VideoMatrixR {
    /// Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MatrixBase {
        match self.bits {
            0 => MatrixBase::Offset0000,
            1 => MatrixBase::Offset0400,
            2 => MatrixBase::Offset0800,
            3 => MatrixBase::Offset0c00,
            4 => MatrixBase::Offset1000,
            5 => MatrixBase::Offset1400,
            6 => MatrixBase::Offset1800,
            7 => MatrixBase::Offset1c00,
            8 => MatrixBase::Offset2000,
            9 => MatrixBase::Offset2400,
            10 => MatrixBase::Offset2800,
            11 => MatrixBase::Offset2c00,
            12 => MatrixBase::Offset3000,
            13 => MatrixBase::Offset3400,
            14 => MatrixBase::Offset3800,
            15 => MatrixBase::Offset3c00,
            _ => unreachable!(),
        }
    }

    /// Screen matrix at offset 0x0000 within the VIC bank
    #[inline(always)]
    pub fn is_offset_0000(&self) -> bool { *self == MatrixBase::Offset0000 }

    /// Screen matrix at offset 0x0400 within the VIC bank
    #[inline(always)]
    pub fn is_offset_0400(&self) -> bool { *self == MatrixBase::Offset0400 }

    /// Screen matrix at offset 0x0800 within the VIC bank
    #[inline(always)]
    pub fn is_offset_0800(&self) -> bool { *self == MatrixBase::Offset0800 }

    /// Screen matrix at offset 0x0C00 within the VIC bank
    #[inline(always)]
    pub fn is_offset_0c00(&self) -> bool { *self == MatrixBase::Offset0c00 }

    /// Screen matrix at offset 0x1000 within the VIC bank
    #[inline(always)]
    pub fn is_offset_1000(&self) -> bool { *self == MatrixBase::Offset1000 }

    /// Screen matrix at offset 0x1400 within the VIC bank
    #[inline(always)]
    pub fn is_offset_1400(&self) -> bool { *self == MatrixBase::Offset1400 }

    /// Screen matrix at offset 0x1800 within the VIC bank
    #[inline(always)]
    pub fn is_offset_1800(&self) -> bool { *self == MatrixBase::Offset1800 }

    /// Screen matrix at offset 0x1C00 within the VIC bank
    #[inline(always)]
    pub fn is_offset_1c00(&self) -> bool { *self == MatrixBase::Offset1c00 }

    /// Screen matrix at offset 0x2000 within the VIC bank
    #[inline(always)]
    pub fn is_offset_2000(&self) -> bool { *self == MatrixBase::Offset2000 }

    /// Screen matrix at offset 0x2400 within the VIC bank
    #[inline(always)]
    pub fn is_offset_2400(&self) -> bool { *self == MatrixBase::Offset2400 }

    /// Screen matrix at offset 0x2800 within the VIC bank
    #[inline(always)]
    pub fn is_offset_2800(&self) -> bool { *self == MatrixBase::Offset2800 }

    /// Screen matrix at offset 0x2C00 within the VIC bank
    #[inline(always)]
    pub fn is_offset_2c00(&self) -> bool { *self == MatrixBase::Offset2c00 }

    /// Screen matrix at offset 0x3000 within the VIC bank
    #[inline(always)]
    pub fn is_offset_3000(&self) -> bool { *self == MatrixBase::Offset3000 }

    /// Screen matrix at offset 0x3400 within the VIC bank
    #[inline(always)]
    pub fn is_offset_3400(&self) -> bool { *self == MatrixBase::Offset3400 }

    /// Screen matrix at offset 0x3800 within the VIC bank
    #[inline(always)]
    pub fn is_offset_3800(&self) -> bool { *self == MatrixBase::Offset3800 }

    /// Screen matrix at offset 0x3C00 within the VIC bank
    #[inline(always)]
    pub fn is_offset_3c00(&self) -> bool { *self == MatrixBase::Offset3c00 }
}
/// Field `VIDEO_MATRIX` writer - Video matrix (screen) base, in 1 KiB steps from the base of the current VIC bank (register bits 7:4). [Mapping the C64](https://archive.org/details/Compute_s_Mapping_the_Commodore_64) quotes this as a whole-register value; divide the book value by 16 to get the field value, or use the named offsets below.
pub type VideoMatrixW<'a, REG> = crate::FieldWriter<'a, REG, 4, MatrixBase, crate::Safe>;
impl<'a, REG> VideoMatrixW<'a, REG>
    where REG: crate::Writable + crate::RegisterSpec,
          REG::Ux: From<u8>,
{
    /// Screen matrix at offset 0x0000 within the VIC bank
    #[inline(always)]
    pub fn offset_0000(self) -> &'a mut crate::W<REG> { self.variant(MatrixBase::Offset0000) }

    /// Screen matrix at offset 0x0400 within the VIC bank
    #[inline(always)]
    pub fn offset_0400(self) -> &'a mut crate::W<REG> { self.variant(MatrixBase::Offset0400) }

    /// Screen matrix at offset 0x0800 within the VIC bank
    #[inline(always)]
    pub fn offset_0800(self) -> &'a mut crate::W<REG> { self.variant(MatrixBase::Offset0800) }

    /// Screen matrix at offset 0x0C00 within the VIC bank
    #[inline(always)]
    pub fn offset_0c00(self) -> &'a mut crate::W<REG> { self.variant(MatrixBase::Offset0c00) }

    /// Screen matrix at offset 0x1000 within the VIC bank
    #[inline(always)]
    pub fn offset_1000(self) -> &'a mut crate::W<REG> { self.variant(MatrixBase::Offset1000) }

    /// Screen matrix at offset 0x1400 within the VIC bank
    #[inline(always)]
    pub fn offset_1400(self) -> &'a mut crate::W<REG> { self.variant(MatrixBase::Offset1400) }

    /// Screen matrix at offset 0x1800 within the VIC bank
    #[inline(always)]
    pub fn offset_1800(self) -> &'a mut crate::W<REG> { self.variant(MatrixBase::Offset1800) }

    /// Screen matrix at offset 0x1C00 within the VIC bank
    #[inline(always)]
    pub fn offset_1c00(self) -> &'a mut crate::W<REG> { self.variant(MatrixBase::Offset1c00) }

    /// Screen matrix at offset 0x2000 within the VIC bank
    #[inline(always)]
    pub fn offset_2000(self) -> &'a mut crate::W<REG> { self.variant(MatrixBase::Offset2000) }

    /// Screen matrix at offset 0x2400 within the VIC bank
    #[inline(always)]
    pub fn offset_2400(self) -> &'a mut crate::W<REG> { self.variant(MatrixBase::Offset2400) }

    /// Screen matrix at offset 0x2800 within the VIC bank
    #[inline(always)]
    pub fn offset_2800(self) -> &'a mut crate::W<REG> { self.variant(MatrixBase::Offset2800) }

    /// Screen matrix at offset 0x2C00 within the VIC bank
    #[inline(always)]
    pub fn offset_2c00(self) -> &'a mut crate::W<REG> { self.variant(MatrixBase::Offset2c00) }

    /// Screen matrix at offset 0x3000 within the VIC bank
    #[inline(always)]
    pub fn offset_3000(self) -> &'a mut crate::W<REG> { self.variant(MatrixBase::Offset3000) }

    /// Screen matrix at offset 0x3400 within the VIC bank
    #[inline(always)]
    pub fn offset_3400(self) -> &'a mut crate::W<REG> { self.variant(MatrixBase::Offset3400) }

    /// Screen matrix at offset 0x3800 within the VIC bank
    #[inline(always)]
    pub fn offset_3800(self) -> &'a mut crate::W<REG> { self.variant(MatrixBase::Offset3800) }

    /// Screen matrix at offset 0x3C00 within the VIC bank
    #[inline(always)]
    pub fn offset_3c00(self) -> &'a mut crate::W<REG> { self.variant(MatrixBase::Offset3c00) }
}
impl R {
    /// Bits 1:3 - Character/bitmap base, in 2 KiB steps from the base of the current VIC bank (register bits 3:1). [Mapping the C64](https://archive.org/details/Compute_s_Mapping_the_Commodore_64) quotes this as a whole-register value; halve the book value to get the field value, or use the named offsets below.
    #[inline(always)]
    pub fn char_base(&self) -> CharBaseR { CharBaseR::new((self.bits >> 1) & 7) }

    /// Bits 4:7 - Video matrix (screen) base, in 1 KiB steps from the base of the current VIC bank (register bits 7:4). [Mapping the C64](https://archive.org/details/Compute_s_Mapping_the_Commodore_64) quotes this as a whole-register value; divide the book value by 16 to get the field value, or use the named offsets below.
    #[inline(always)]
    pub fn video_matrix(&self) -> VideoMatrixR { VideoMatrixR::new((self.bits >> 4) & 0x0f) }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VMCSB")
         .field("char_base", &self.char_base())
         .field("video_matrix", &self.video_matrix())
         .finish()
    }
}
impl W {
    /// Bits 1:3 - Character/bitmap base, in 2 KiB steps from the base of the current VIC bank (register bits 3:1). [Mapping the C64](https://archive.org/details/Compute_s_Mapping_the_Commodore_64) quotes this as a whole-register value; halve the book value to get the field value, or use the named offsets below.
    #[inline(always)]
    pub fn char_base(&mut self) -> CharBaseW<'_, VmcsbSpec> { CharBaseW::new(self, 1) }

    /// Bits 4:7 - Video matrix (screen) base, in 1 KiB steps from the base of the current VIC bank (register bits 7:4). [Mapping the C64](https://archive.org/details/Compute_s_Mapping_the_Commodore_64) quotes this as a whole-register value; divide the book value by 16 to get the field value, or use the named offsets below.
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
