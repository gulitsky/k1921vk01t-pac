#[doc = "Reader of register INTPOLCLR"]
pub type R = crate::R<u32, super::INTPOLCLR>;
#[doc = "Writer for register INTPOLCLR"]
pub type W = crate::W<u32, super::INTPOLCLR>;
#[doc = "Register INTPOLCLR `reset()`'s with value 0"]
impl crate::ResetValue for super::INTPOLCLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `INTPOLCLR`"]
pub type INTPOLCLR_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `INTPOLCLR`"]
pub struct INTPOLCLR_W<'a> {
    w: &'a mut W,
}
impl<'a> INTPOLCLR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn intpolclr(&self) -> INTPOLCLR_R {
        INTPOLCLR_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn intpolclr(&mut self) -> INTPOLCLR_W {
        INTPOLCLR_W { w: self }
    }
}
