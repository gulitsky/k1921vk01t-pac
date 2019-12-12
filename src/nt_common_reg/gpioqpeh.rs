#[doc = "Reader of register GPIOQPEH"]
pub type R = crate::R<u32, super::GPIOQPEH>;
#[doc = "Writer for register GPIOQPEH"]
pub type W = crate::W<u32, super::GPIOQPEH>;
#[doc = "Register GPIOQPEH `reset()`'s with value 0"]
impl crate::ResetValue for super::GPIOQPEH {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SPRDE`"]
pub type SPRDE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SPRDE`"]
pub struct SPRDE_W<'a> {
    w: &'a mut W,
}
impl<'a> SPRDE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `SPRDF`"]
pub type SPRDF_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SPRDF`"]
pub struct SPRDF_W<'a> {
    w: &'a mut W,
}
impl<'a> SPRDF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `SPRDG`"]
pub type SPRDG_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SPRDG`"]
pub struct SPRDG_W<'a> {
    w: &'a mut W,
}
impl<'a> SPRDG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `SPRDH`"]
pub type SPRDH_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SPRDH`"]
pub struct SPRDH_W<'a> {
    w: &'a mut W,
}
impl<'a> SPRDH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn sprde(&self) -> SPRDE_R {
        SPRDE_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn sprdf(&self) -> SPRDF_R {
        SPRDF_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn sprdg(&self) -> SPRDG_R {
        SPRDG_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn sprdh(&self) -> SPRDH_R {
        SPRDH_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn sprde(&mut self) -> SPRDE_W {
        SPRDE_W { w: self }
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn sprdf(&mut self) -> SPRDF_W {
        SPRDF_W { w: self }
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn sprdg(&mut self) -> SPRDG_W {
        SPRDG_W { w: self }
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn sprdh(&mut self) -> SPRDH_W {
        SPRDH_W { w: self }
    }
}
