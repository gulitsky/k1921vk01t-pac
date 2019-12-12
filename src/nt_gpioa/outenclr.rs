#[doc = "Reader of register OUTENCLR"]
pub type R = crate::R<u32, super::OUTENCLR>;
#[doc = "Writer for register OUTENCLR"]
pub type W = crate::W<u32, super::OUTENCLR>;
#[doc = "Register OUTENCLR `reset()`'s with value 0"]
impl crate::ResetValue for super::OUTENCLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `OUTENCLR`"]
pub type OUTENCLR_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `OUTENCLR`"]
pub struct OUTENCLR_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTENCLR_W<'a> {
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
    pub fn outenclr(&self) -> OUTENCLR_R {
        OUTENCLR_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn outenclr(&mut self) -> OUTENCLR_W {
        OUTENCLR_W { w: self }
    }
}
