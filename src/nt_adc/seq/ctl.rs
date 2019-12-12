#[doc = "Reader of register CTL"]
pub type R = crate::R<u32, super::CTL>;
#[doc = "Writer for register CTL"]
pub type W = crate::W<u32, super::CTL>;
#[doc = "Register CTL `reset()`'s with value 0"]
impl crate::ResetValue for super::CTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RCNT`"]
pub type RCNT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RCNT`"]
pub struct RCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> RCNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `ICNT`"]
pub type ICNT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ICNT`"]
pub struct ICNT_W<'a> {
    w: &'a mut W,
}
impl<'a> ICNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "FIFO load threshold for DMA request generation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WMARK_A {
    #[doc = "1: 1 measure for dma request"]
    LEVEL1,
    #[doc = "2: 2 measures for dma request"]
    LEVEL2,
    #[doc = "3: 4 measures for dma request"]
    LEVEL4,
    #[doc = "4: 8 measures for dma request"]
    LEVEL8,
    #[doc = "5: 16 measures for dma request"]
    LEVEL16,
    #[doc = "6: 32 measures for dma request"]
    LEVEL32,
}
impl From<WMARK_A> for u8 {
    #[inline(always)]
    fn from(variant: WMARK_A) -> Self {
        match variant {
            WMARK_A::LEVEL1 => 1,
            WMARK_A::LEVEL2 => 2,
            WMARK_A::LEVEL4 => 3,
            WMARK_A::LEVEL8 => 4,
            WMARK_A::LEVEL16 => 5,
            WMARK_A::LEVEL32 => 6,
        }
    }
}
#[doc = "Reader of field `WMARK`"]
pub type WMARK_R = crate::R<u8, WMARK_A>;
impl WMARK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, WMARK_A> {
        use crate::Variant::*;
        match self.bits {
            1 => Val(WMARK_A::LEVEL1),
            2 => Val(WMARK_A::LEVEL2),
            3 => Val(WMARK_A::LEVEL4),
            4 => Val(WMARK_A::LEVEL8),
            5 => Val(WMARK_A::LEVEL16),
            6 => Val(WMARK_A::LEVEL32),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL1`"]
    #[inline(always)]
    pub fn is_level1(&self) -> bool {
        *self == WMARK_A::LEVEL1
    }
    #[doc = "Checks if the value of the field is `LEVEL2`"]
    #[inline(always)]
    pub fn is_level2(&self) -> bool {
        *self == WMARK_A::LEVEL2
    }
    #[doc = "Checks if the value of the field is `LEVEL4`"]
    #[inline(always)]
    pub fn is_level4(&self) -> bool {
        *self == WMARK_A::LEVEL4
    }
    #[doc = "Checks if the value of the field is `LEVEL8`"]
    #[inline(always)]
    pub fn is_level8(&self) -> bool {
        *self == WMARK_A::LEVEL8
    }
    #[doc = "Checks if the value of the field is `LEVEL16`"]
    #[inline(always)]
    pub fn is_level16(&self) -> bool {
        *self == WMARK_A::LEVEL16
    }
    #[doc = "Checks if the value of the field is `LEVEL32`"]
    #[inline(always)]
    pub fn is_level32(&self) -> bool {
        *self == WMARK_A::LEVEL32
    }
}
#[doc = "Write proxy for field `WMARK`"]
pub struct WMARK_W<'a> {
    w: &'a mut W,
}
impl<'a> WMARK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WMARK_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "1 measure for dma request"]
    #[inline(always)]
    pub fn level1(self) -> &'a mut W {
        self.variant(WMARK_A::LEVEL1)
    }
    #[doc = "2 measures for dma request"]
    #[inline(always)]
    pub fn level2(self) -> &'a mut W {
        self.variant(WMARK_A::LEVEL2)
    }
    #[doc = "4 measures for dma request"]
    #[inline(always)]
    pub fn level4(self) -> &'a mut W {
        self.variant(WMARK_A::LEVEL4)
    }
    #[doc = "8 measures for dma request"]
    #[inline(always)]
    pub fn level8(self) -> &'a mut W {
        self.variant(WMARK_A::LEVEL8)
    }
    #[doc = "16 measures for dma request"]
    #[inline(always)]
    pub fn level16(self) -> &'a mut W {
        self.variant(WMARK_A::LEVEL16)
    }
    #[doc = "32 measures for dma request"]
    #[inline(always)]
    pub fn level32(self) -> &'a mut W {
        self.variant(WMARK_A::LEVEL32)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 24)) | (((value as u32) & 0x07) << 24);
        self.w
    }
}
#[doc = "Reader of field `DMAEN`"]
pub type DMAEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMAEN`"]
pub struct DMAEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAEN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Number of ADC restarts by seqensor"]
    #[inline(always)]
    pub fn rcnt(&self) -> RCNT_R {
        RCNT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Number of ADC restarts for interrupt generation"]
    #[inline(always)]
    pub fn icnt(&self) -> ICNT_R {
        ICNT_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:26 - FIFO load threshold for DMA request generation"]
    #[inline(always)]
    pub fn wmark(&self) -> WMARK_R {
        WMARK_R::new(((self.bits >> 24) & 0x07) as u8)
    }
    #[doc = "Bit 28 - Enable DMA use"]
    #[inline(always)]
    pub fn dmaen(&self) -> DMAEN_R {
        DMAEN_R::new(((self.bits >> 28) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Number of ADC restarts by seqensor"]
    #[inline(always)]
    pub fn rcnt(&mut self) -> RCNT_W {
        RCNT_W { w: self }
    }
    #[doc = "Bits 16:23 - Number of ADC restarts for interrupt generation"]
    #[inline(always)]
    pub fn icnt(&mut self) -> ICNT_W {
        ICNT_W { w: self }
    }
    #[doc = "Bits 24:26 - FIFO load threshold for DMA request generation"]
    #[inline(always)]
    pub fn wmark(&mut self) -> WMARK_W {
        WMARK_W { w: self }
    }
    #[doc = "Bit 28 - Enable DMA use"]
    #[inline(always)]
    pub fn dmaen(&mut self) -> DMAEN_W {
        DMAEN_W { w: self }
    }
}
