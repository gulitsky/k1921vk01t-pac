#[doc = "Reader of register CMPA"]
pub type R = crate::R<u32, super::CMPA>;
#[doc = "Writer for register CMPA"]
pub type W = crate::W<u32, super::CMPA>;
#[doc = "Register CMPA `reset()`'s with value 0"]
impl crate::ResetValue for super::CMPA {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CMPAHR`"]
pub type CMPAHR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CMPAHR`"]
pub struct CMPAHR_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPAHR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `CMPA`"]
pub type CMPA_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CMPA`"]
pub struct CMPA_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn cmpahr(&self) -> CMPAHR_R {
        CMPAHR_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn cmpa(&self) -> CMPA_R {
        CMPA_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn cmpahr(&mut self) -> CMPAHR_W {
        CMPAHR_W { w: self }
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn cmpa(&mut self) -> CMPA_W {
        CMPA_W { w: self }
    }
}
