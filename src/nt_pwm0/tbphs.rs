#[doc = "Reader of register TBPHS"]
pub type R = crate::R<u32, super::TBPHS>;
#[doc = "Writer for register TBPHS"]
pub type W = crate::W<u32, super::TBPHS>;
#[doc = "Register TBPHS `reset()`'s with value 0"]
impl crate::ResetValue for super::TBPHS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TBPHSHR`"]
pub type TBPHSHR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TBPHSHR`"]
pub struct TBPHSHR_W<'a> {
    w: &'a mut W,
}
impl<'a> TBPHSHR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `TBPHS`"]
pub type TBPHS_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `TBPHS`"]
pub struct TBPHS_W<'a> {
    w: &'a mut W,
}
impl<'a> TBPHS_W<'a> {
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
    pub fn tbphshr(&self) -> TBPHSHR_R {
        TBPHSHR_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn tbphs(&self) -> TBPHS_R {
        TBPHS_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn tbphshr(&mut self) -> TBPHSHR_W {
        TBPHSHR_W { w: self }
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn tbphs(&mut self) -> TBPHS_W {
        TBPHS_W { w: self }
    }
}
