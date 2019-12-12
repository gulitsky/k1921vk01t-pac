#[doc = "Reader of register QCTMR"]
pub type R = crate::R<u32, super::QCTMR>;
#[doc = "Writer for register QCTMR"]
pub type W = crate::W<u32, super::QCTMR>;
#[doc = "Register QCTMR `reset()`'s with value 0"]
impl crate::ResetValue for super::QCTMR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `QCTMR`"]
pub type QCTMR_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `QCTMR`"]
pub struct QCTMR_W<'a> {
    w: &'a mut W,
}
impl<'a> QCTMR_W<'a> {
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
    pub fn qctmr(&self) -> QCTMR_R {
        QCTMR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn qctmr(&mut self) -> QCTMR_W {
        QCTMR_W { w: self }
    }
}
