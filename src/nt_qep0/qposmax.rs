#[doc = "Reader of register QPOSMAX"]
pub type R = crate::R<u32, super::QPOSMAX>;
#[doc = "Writer for register QPOSMAX"]
pub type W = crate::W<u32, super::QPOSMAX>;
#[doc = "Register QPOSMAX `reset()`'s with value 0"]
impl crate::ResetValue for super::QPOSMAX {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `QPOSMAX`"]
pub type QPOSMAX_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `QPOSMAX`"]
pub struct QPOSMAX_W<'a> {
    w: &'a mut W,
}
impl<'a> QPOSMAX_W<'a> {
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
    pub fn qposmax(&self) -> QPOSMAX_R {
        QPOSMAX_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn qposmax(&mut self) -> QPOSMAX_W {
        QPOSMAX_W { w: self }
    }
}
