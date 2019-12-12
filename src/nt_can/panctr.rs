#[doc = "Reader of register PANCTR"]
pub type R = crate::R<u32, super::PANCTR>;
#[doc = "Writer for register PANCTR"]
pub type W = crate::W<u32, super::PANCTR>;
#[doc = "Register PANCTR `reset()`'s with value 0"]
impl crate::ResetValue for super::PANCTR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PANCMD`"]
pub type PANCMD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PANCMD`"]
pub struct PANCMD_W<'a> {
    w: &'a mut W,
}
impl<'a> PANCMD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `BUSY`"]
pub type BUSY_R = crate::R<bool, bool>;
#[doc = "Reader of field `RBUSY`"]
pub type RBUSY_R = crate::R<bool, bool>;
#[doc = "Reader of field `PANAR1`"]
pub type PANAR1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PANAR1`"]
pub struct PANAR1_W<'a> {
    w: &'a mut W,
}
impl<'a> PANAR1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `PANAR2`"]
pub type PANAR2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PANAR2`"]
pub struct PANAR2_W<'a> {
    w: &'a mut W,
}
impl<'a> PANAR2_W<'a> {
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
    pub fn pancmd(&self) -> PANCMD_R {
        PANCMD_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - Busy flag panels arguments (waiting to be written at the end of the command)"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Busy flag panels arguments (running the command list, the result of which will be recorded in PANAR1 and PANAR2)"]
    #[inline(always)]
    pub fn rbusy(&self) -> RBUSY_R {
        RBUSY_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bits 16:23 - Panel argument 8"]
    #[inline(always)]
    pub fn panar1(&self) -> PANAR1_R {
        PANAR1_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Panel argument 9"]
    #[inline(always)]
    pub fn panar2(&self) -> PANAR2_R {
        PANAR2_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn pancmd(&mut self) -> PANCMD_W {
        PANCMD_W { w: self }
    }
    #[doc = "Bits 16:23 - Panel argument 8"]
    #[inline(always)]
    pub fn panar1(&mut self) -> PANAR1_W {
        PANAR1_W { w: self }
    }
    #[doc = "Bits 24:31 - Panel argument 9"]
    #[inline(always)]
    pub fn panar2(&mut self) -> PANAR2_W {
        PANAR2_W { w: self }
    }
}
