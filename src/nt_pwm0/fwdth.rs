#[doc = "Reader of register FWDTH"]
pub type R = crate::R<u32, super::FWDTH>;
#[doc = "Writer for register FWDTH"]
pub type W = crate::W<u32, super::FWDTH>;
#[doc = "Register FWDTH `reset()`'s with value 0"]
impl crate::ResetValue for super::FWDTH {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FWDTH`"]
pub type FWDTH_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FWDTH`"]
pub struct FWDTH_W<'a> {
    w: &'a mut W,
}
impl<'a> FWDTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn fwdth(&self) -> FWDTH_R {
        FWDTH_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn fwdth(&mut self) -> FWDTH_W {
        FWDTH_W { w: self }
    }
}
