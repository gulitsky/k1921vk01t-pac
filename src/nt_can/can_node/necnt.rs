#[doc = "Reader of register NECNT"]
pub type R = crate::R<u32, super::NECNT>;
#[doc = "Writer for register NECNT"]
pub type W = crate::W<u32, super::NECNT>;
#[doc = "Register NECNT `reset()`'s with value 0"]
impl crate::ResetValue for super::NECNT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `REC`"]
pub type REC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `REC`"]
pub struct REC_W<'a> {
    w: &'a mut W,
}
impl<'a> REC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `TEC`"]
pub type TEC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TEC`"]
pub struct TEC_W<'a> {
    w: &'a mut W,
}
impl<'a> TEC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `EWRNLVL`"]
pub type EWRNLVL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EWRNLVL`"]
pub struct EWRNLVL_W<'a> {
    w: &'a mut W,
}
impl<'a> EWRNLVL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `LETD`"]
pub type LETD_R = crate::R<bool, bool>;
#[doc = "Reader of field `LEINC`"]
pub type LEINC_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn rec(&self) -> REC_R {
        REC_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn tec(&self) -> TEC_R {
        TEC_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn ewrnlvl(&self) -> EWRNLVL_R {
        EWRNLVL_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - Flag last transmission errors"]
    #[inline(always)]
    pub fn letd(&self) -> LETD_R {
        LETD_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Indicator increment at the last error"]
    #[inline(always)]
    pub fn leinc(&self) -> LEINC_R {
        LEINC_R::new(((self.bits >> 25) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn rec(&mut self) -> REC_W {
        REC_W { w: self }
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn tec(&mut self) -> TEC_W {
        TEC_W { w: self }
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn ewrnlvl(&mut self) -> EWRNLVL_W {
        EWRNLVL_W { w: self }
    }
}
