#[doc = "Reader of register CTL2"]
pub type R = crate::R<u32, super::CTL2>;
#[doc = "Writer for register CTL2"]
pub type W = crate::W<u32, super::CTL2>;
#[doc = "Register CTL2 `reset()`'s with value 0"]
impl crate::ResetValue for super::CTL2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `S10ADR`"]
pub type S10ADR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `S10ADR`"]
pub struct S10ADR_W<'a> {
    w: &'a mut W,
}
impl<'a> S10ADR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "Reader of field `S10EN`"]
pub type S10EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `S10EN`"]
pub struct S10EN_W<'a> {
    w: &'a mut W,
}
impl<'a> S10EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `HSDIV`"]
pub type HSDIV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HSDIV`"]
pub struct HSDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> HSDIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn s10adr(&self) -> S10ADR_R {
        S10ADR_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 3 - Bit enabled 10-bit addressing slave"]
    #[inline(always)]
    pub fn s10en(&self) -> S10EN_R {
        S10EN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn hsdiv(&self) -> HSDIV_R {
        HSDIV_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn s10adr(&mut self) -> S10ADR_W {
        S10ADR_W { w: self }
    }
    #[doc = "Bit 3 - Bit enabled 10-bit addressing slave"]
    #[inline(always)]
    pub fn s10en(&mut self) -> S10EN_W {
        S10EN_W { w: self }
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn hsdiv(&mut self) -> HSDIV_W {
        HSDIV_W { w: self }
    }
}
