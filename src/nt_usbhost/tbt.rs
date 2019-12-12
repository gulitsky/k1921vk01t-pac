#[doc = "Reader of register TBT"]
pub type R = crate::R<u32, super::TBT>;
#[doc = "Writer for register TBT"]
pub type W = crate::W<u32, super::TBT>;
#[doc = "Register TBT `reset()`'s with value 0"]
impl crate::ResetValue for super::TBT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TOTALBTRANS`"]
pub type TOTALBTRANS_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `TOTALBTRANS`"]
pub struct TOTALBTRANS_W<'a> {
    w: &'a mut W,
}
impl<'a> TOTALBTRANS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x000f_ffff) | ((value as u32) & 0x000f_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:19 - Total Byte transfer"]
    #[inline(always)]
    pub fn totalbtrans(&self) -> TOTALBTRANS_R {
        TOTALBTRANS_R::new((self.bits & 0x000f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:19 - Total Byte transfer"]
    #[inline(always)]
    pub fn totalbtrans(&mut self) -> TOTALBTRANS_W {
        TOTALBTRANS_W { w: self }
    }
}
