#[doc = "Reader of register ETPS"]
pub type R = crate::R<u32, super::ETPS>;
#[doc = "Writer for register ETPS"]
pub type W = crate::W<u32, super::ETPS>;
#[doc = "Register ETPS `reset()`'s with value 0"]
impl crate::ResetValue for super::ETPS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `INTPRD`"]
pub type INTPRD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `INTPRD`"]
pub struct INTPRD_W<'a> {
    w: &'a mut W,
}
impl<'a> INTPRD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `INTCNT`"]
pub type INTCNT_R = crate::R<u8, u8>;
#[doc = "Reader of field `SOCAPRD`"]
pub type SOCAPRD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SOCAPRD`"]
pub struct SOCAPRD_W<'a> {
    w: &'a mut W,
}
impl<'a> SOCAPRD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Reader of field `SOCACNT`"]
pub type SOCACNT_R = crate::R<u8, u8>;
#[doc = "Reader of field `SOCBPRD`"]
pub type SOCBPRD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SOCBPRD`"]
pub struct SOCBPRD_W<'a> {
    w: &'a mut W,
}
impl<'a> SOCBPRD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "Reader of field `SOCBCNT`"]
pub type SOCBCNT_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn intprd(&self) -> INTPRD_R {
        INTPRD_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn intcnt(&self) -> INTCNT_R {
        INTCNT_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn socaprd(&self) -> SOCAPRD_R {
        SOCAPRD_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn socacnt(&self) -> SOCACNT_R {
        SOCACNT_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn socbprd(&self) -> SOCBPRD_R {
        SOCBPRD_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn socbcnt(&self) -> SOCBCNT_R {
        SOCBCNT_R::new(((self.bits >> 14) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn intprd(&mut self) -> INTPRD_W {
        INTPRD_W { w: self }
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn socaprd(&mut self) -> SOCAPRD_W {
        SOCAPRD_W { w: self }
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn socbprd(&mut self) -> SOCBPRD_W {
        SOCBPRD_W { w: self }
    }
}
