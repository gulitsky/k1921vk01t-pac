#[doc = "Reader of register GPIOQPAD"]
pub type R = crate::R<u32, super::GPIOQPAD>;
#[doc = "Writer for register GPIOQPAD"]
pub type W = crate::W<u32, super::GPIOQPAD>;
#[doc = "Register GPIOQPAD `reset()`'s with value 0"]
impl crate::ResetValue for super::GPIOQPAD {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SPRDA`"]
pub type SPRDA_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SPRDA`"]
pub struct SPRDA_W<'a> {
    w: &'a mut W,
}
impl<'a> SPRDA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `SPRDB`"]
pub type SPRDB_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SPRDB`"]
pub struct SPRDB_W<'a> {
    w: &'a mut W,
}
impl<'a> SPRDB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `SPRDC`"]
pub type SPRDC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SPRDC`"]
pub struct SPRDC_W<'a> {
    w: &'a mut W,
}
impl<'a> SPRDC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `SPRDD`"]
pub type SPRDD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SPRDD`"]
pub struct SPRDD_W<'a> {
    w: &'a mut W,
}
impl<'a> SPRDD_W<'a> {
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
    pub fn sprda(&self) -> SPRDA_R {
        SPRDA_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn sprdb(&self) -> SPRDB_R {
        SPRDB_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn sprdc(&self) -> SPRDC_R {
        SPRDC_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn sprdd(&self) -> SPRDD_R {
        SPRDD_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn sprda(&mut self) -> SPRDA_W {
        SPRDA_W { w: self }
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn sprdb(&mut self) -> SPRDB_W {
        SPRDB_W { w: self }
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn sprdc(&mut self) -> SPRDC_W {
        SPRDC_W { w: self }
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn sprdd(&mut self) -> SPRDD_W {
        SPRDD_W { w: self }
    }
}
