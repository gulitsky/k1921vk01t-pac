#[doc = "Reader of register CEP_IN_XFRCNT"]
pub type R = crate::R<u32, super::CEP_IN_XFRCNT>;
#[doc = "Writer for register CEP_IN_XFRCNT"]
pub type W = crate::W<u32, super::CEP_IN_XFRCNT>;
#[doc = "Register CEP_IN_XFRCNT `reset()`'s with value 0"]
impl crate::ResetValue for super::CEP_IN_XFRCNT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DATACOUNT`"]
pub type DATACOUNT_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `DATACOUNT`"]
pub struct DATACOUNT_W<'a> {
    w: &'a mut W,
}
impl<'a> DATACOUNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - The number of bytes to be sent in response to a label IN"]
    #[inline(always)]
    pub fn datacount(&self) -> DATACOUNT_R {
        DATACOUNT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - The number of bytes to be sent in response to a label IN"]
    #[inline(always)]
    pub fn datacount(&mut self) -> DATACOUNT_W {
        DATACOUNT_W { w: self }
    }
}
