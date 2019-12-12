#[doc = "Reader of register QWDTMR"]
pub type R = crate::R<u32, super::QWDTMR>;
#[doc = "Writer for register QWDTMR"]
pub type W = crate::W<u32, super::QWDTMR>;
#[doc = "Register QWDTMR `reset()`'s with value 0"]
impl crate::ResetValue for super::QWDTMR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `QWDTMR`"]
pub type QWDTMR_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `QWDTMR`"]
pub struct QWDTMR_W<'a> {
    w: &'a mut W,
}
impl<'a> QWDTMR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn qwdtmr(&self) -> QWDTMR_R {
        QWDTMR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn qwdtmr(&mut self) -> QWDTMR_W {
        QWDTMR_W { w: self }
    }
}
